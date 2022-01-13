use crate::dom;
use crate::editor::{
    component,
    component::components,
    component::{lumped, probe, source},
    entity,
    entity::Entity,
    mouse, property, styles, wire,
};
use crate::intrinsics::*;
use crate::simulation;
use std::collections::BTreeMap;
use wasm_bindgen::JsCast;

pub struct Entities {
    pub entities: Vec<Box<dyn entity::Entity>>,
    selected: Vec<usize>,
    hovered: Vec<usize>,
    selected_connections: Vec<(usize, usize)>,
    pub were_some_selected: bool,
    pub floating_wire: Option<wire::Wire>,
    pub floating_component: Option<component::Component>,
}

impl Entities {
    pub fn new() -> Self {
        let entities: Vec<Box<dyn entity::Entity>> = vec![
            Box::new(lumped::resistor(Point::new(200.0, 200.0), "R0".to_string())),
            Box::new(lumped::capacitor(
                Point::new(300.0, 200.0),
                "C0".to_string(),
            )),
            Box::new(source::voltage_ac(
                Point::new(100.0, 195.0),
                "Vin".to_string(),
            )),
            //Box::new(lumped::node(Point::new(200.0, 163.0), "n0".to_string())),
            //Box::new(lumped::node(Point::new(290.0, 203.0), "n1".to_string())),
            Box::new(lumped::ground(Point::new(170.0, 210.0), "0".to_string())),
            Box::new(probe::voltmeter(Point::new(300.0, 180.0), "P0".to_string())),
            Box::new(wire::Wire::new(
                Point::new(180.0, 210.0),
                Point::new(200.0, 200.0),
            )),
            Box::new(wire::Wire::new(
                Point::new(280.0, 210.0),
                Point::new(300.0, 210.0),
            )),
            Box::new(wire::Wire::new(
                Point::new(380.0, 210.0),
                Point::new(380.0, 170.0),
            )),
            Box::new(wire::Wire::new(
                Point::new(380.0, 170.0),
                Point::new(100.0, 170.0),
            )),
            Box::new(wire::Wire::new(
                Point::new(100.0, 170.0),
                Point::new(100.0, 210.0),
            )),
            // Box::new(lumped::resistor(Point::new(100.0, 50.0), format!("R{}", 0))),
            // Box::new(lumped::capacitor(Point::new(100.0, 110.0), format!("C{}", 1))),
            // Box::new(source::voltage_dc(Point::new(100.0, 170.0), format!("vcc{}", 2))),
            // Box::new(lumped::node(Point::new(100.0, 70.0), "n0".to_string())),
            // Box::new(lumped::ground(Point::new(160.0, 70.0), "gnd".to_string())),
            // Box::new(wire::Wire::new(Point::new(100.0, 60.0), Point::new(100.0, 180.0), wire::color())),
            // Box::new(wire::Wire::new(Point::new(180.0, 60.0), Point::new(180.0, 180.0), wire::color())),
        ];

        //simulation::dialog::update_probes(&entities);
        Self {
            entities,
            selected: Vec::new(),
            hovered: Vec::new(),
            selected_connections: Vec::new(),
            were_some_selected: false,
            floating_wire: None,
            floating_component: None,
        }
    }

    pub fn drag(&mut self, mouse: &mouse::Mouse) {
        for idx in &self.selected {
            self.entities[*idx].drag(&mouse);
        }
        self.update_connections();
    }

    pub fn select(&mut self, mouse: &mut mouse::Mouse) {
        mouse.action = mouse::MouseAction::MoveView;
        self.were_some_selected = self.selected.len() > 0;
        self.update_selected_entity_properties();
        self.update_hovered(&*mouse);
        self.selected = self.hovered.clone();
        let mut idx = 0;
        for entity in &mut self.entities {
            if self.selected.contains(&idx) {
                entity.set_selected_offset(mouse.scene_pos - entity.origin());
                entity.set_is_selected(true);
                mouse.action = mouse::MouseAction::MoveEntity;
                //self.selected.push(idx);
                self.were_some_selected = true;
            } else {
                entity.set_is_selected(false);
            }
            idx += 1;
        }
        if self.selected.len() == 1 && !self.entities[self.selected[0]].is_wire() {
            component::dialogs::load_properties_dialog(&self.entities[self.selected[0]]);
        } else {
            component::dialogs::empty_properties_dialog();
        }
    }

    pub fn update_hovered(&mut self, mouse: &mouse::Mouse) {
        let mut idx = 0;
        self.hovered = Vec::new();
        // If the mouse is hovering a connection, we change the cursor not for a pointer but for a
        // + sign, indicating that the user can start a wire from this point.
        let mut hovered_connections = Vec::new();
        for entity in &mut self.entities {
            entity.set_is_hovered(false);
            if entity.collide_with_point(mouse.scene_pos)
                && self.floating_wire.is_none()
                && self.floating_component.is_none()
            {
                if entity
                    .connections_collide_with_point(mouse.scene_pos)
                    .is_none()
                {
                    entity.set_is_hovered(true);
                    self.hovered.push(idx);
                } else {
                    hovered_connections.push(idx);
                }
            }
            idx += 1;
        }
        let cursor_value = if hovered_connections.len() > 0 {
            "crosshair"
        } else if self.hovered.len() > 0 {
            "pointer"
        } else {
            "default"
        };
        if hovered_connections.len() > 0 {
            self.hovered = Vec::new();
        }
        dom::select("body")
            .dyn_into::<web_sys::HtmlElement>()
            .map_err(|_| ())
            .unwrap()
            .style()
            .set_property("cursor", cursor_value)
            .unwrap();
    }

    pub fn unselect(&mut self) {
        self.update_selected_entity_properties();
        self.were_some_selected = false;
        self.selected = vec![];
        for entity in &mut self.entities {
            entity.set_is_selected(false);
        }
    }

    /// Select all connections point that are under the mouse.
    pub fn select_connections(&mut self, point: Point) {
        self.selected_connections = Vec::new();
        let mut idx = 0;
        for entity in &mut self.entities {
            let colliding_connections = entity.connections_collide_with_point(point);
            if let Some(connection_idx) = colliding_connections {
                self.selected_connections.push((idx, connection_idx));
            }
            idx += 1;
        }
    }

    /// Add a floating component to the screen were it is not yet placed down but is following the
    /// mouse pointer. When the user click the mouse, the component will actually be placed down.
    pub fn add_floating_component(&mut self, component: &components::Components) {
        let mut new_component = component.generate(self.entities.len());
        new_component.set_is_selected(true);
        new_component.set_selected_offset(Point::new(
            new_component.size().w / 2.0,
            new_component.size().h / 2.0,
        ));
        self.selected = vec![];
        self.floating_component = Some(new_component);
    }

    /// Actually add the floating component to the editor. Remove the floating component.
    pub fn add_component(&mut self) {
        if let Some(component) = &mut self.floating_component {
            component.set_is_selected(false);
            self.entities
                .push(Box::new(component.clone()) as Box<dyn entity::Entity>);
            self.floating_component = None;
            self.were_some_selected = true;
            //simulation::dialog::update_probes(&self.entities);
        }
        self.update_connections();
    }

    /// Update the position of the floating component based on the position of the mouse.
    pub fn update_floating_component(&mut self, mouse: &mouse::Mouse) {
        if let Some(component) = &mut self.floating_component {
            component.set_origin(mouse.scene_pos - component.selected_offset());
        }
    }

    /// Add a floating wire to the screen were it is not yet placed down but is following the
    /// mouse pointer. When the user click the mouse, the wire will actually be placed down.
    pub fn add_floating_wire(&mut self, origin: Point) {
        self.floating_wire = Some(wire::Wire::new(origin, origin));
    }

    /// Actually add the floating wire to the editor. Remove the floating wire.
    pub fn add_wire(&mut self, origin: Point) {
        if let Some(wire) = &self.floating_wire {
            self.entities
                .push(Box::new(wire.clone()) as Box<dyn entity::Entity>);
            let new_origin = wire.origin() + wire.shape().polygones[0][1];
            self.select_connections(new_origin);
            if self.selected_connections.len() == 0 {
                self.add_floating_wire(new_origin);
            } else {
                self.floating_wire = None;
            }
        } else {
            self.add_floating_wire(origin);
        }
        self.update_connections();
    }

    /// Update the shape of the floating wire based on the position of the mouse.
    pub fn update_floating_wire(&mut self, mouse: &mouse::Mouse) {
        if let Some(wire) = &mut self.floating_wire {
            wire.update_shape(wire.origin(), mouse.scene_pos);
        }
    }

    /// Delete all selected entity.
    pub fn delete_selected(&mut self, mouse: &mut mouse::Mouse) {
        self.entities = self
            .entities
            .iter()
            .enumerate()
            .filter(|(idx, _)| !self.selected.contains(idx))
            .map(|(_, entity)| entity.clone())
            .collect::<Vec<Box<dyn entity::Entity>>>();
        self.selected = vec![];
        self.were_some_selected = false;
        if let Some(_) = &mut self.floating_component {
            self.floating_component = None;
            mouse.action = mouse::MouseAction::None;
        }
        self.update_connections();
        component::dialogs::empty_properties_dialog();
        //simulation::dialog::update_probes(&self.entities);
    }

    /// Rotate all selected entities by 90 deg.
    pub fn rotate_selected(&mut self) {
        for idx in &self.selected {
            let entity = &mut self.entities[*idx];
            if entity.is_draggable() {
                entity.rotate();
            }
        }
        if let Some(component) = &mut self.floating_component {
            if component.is_draggable() {
                component.rotate();
            }
        }
        self.update_connections();
    }

    /// Unselect all entities and if a floating component or wire is present, remove it.
    pub fn unselect_all(&mut self) {
        self.floating_component = None;
        self.floating_wire = None;
        self.unselect();
        component::dialogs::empty_properties_dialog();
    }

    /// Draw entities on the screen.
    pub fn draw(&self, context: &web_sys::CanvasRenderingContext2d) {
        // We need to draw the wires first, then the unselected components and finally the selected
        // componenents.
        for entity in &self.entities {
            if entity.is_wire() {
                entity.draw(&context);
            }
        }

        let mut idx = 0;
        for entity in &self.entities {
            if !entity.is_wire() && !self.selected.contains(&idx) {
                entity.draw(&context);
            }
            idx += 1;
        }

        for idx in self.selected.iter() {
            if !self.entities[*idx].is_wire() {
                self.entities[*idx].draw(&context);
            }
        }

        if let Some(wire) = &self.floating_wire {
            wire.draw(&context);
        }
        if let Some(component) = &self.floating_component {
            component.draw(&context);
        }
    }

    pub fn update_connections(&mut self) {
        let mut connections = Vec::new();
        let mut idx = 0;
        for entity in &self.entities {
            if !entity.is_wire() {
                for (conn_idx, conn) in entity.connections().iter().enumerate() {
                    for other_entity in &self.entities {
                        if entity.origin() != other_entity.origin()
                            || entity.shape() != other_entity.shape()
                        {
                            if other_entity.is_wire() {
                                if other_entity.collide_with_point(*conn + entity.origin()) {
                                    connections.push((idx, conn_idx));
                                }
                            } else {
                                for other_conn in other_entity.connections().iter() {
                                    if *other_conn + other_entity.origin()
                                        == *conn + entity.origin()
                                    {
                                        connections.push((idx, conn_idx));
                                    }
                                }
                            }
                        }
                    }
                }
            }
            idx += 1;
        }
        for entity in &mut self.entities {
            entity.reset_connections();
        }
        for connection in connections {
            self.entities[connection.0].set_connection(connection.1, true);
        }
    }

    pub fn update_selected_entity_properties(&mut self) {
        if self.selected.len() == 1 {
            let entity = &mut self.entities[self.selected[0]];
            let mut properties = BTreeMap::new();
            for (&key, current_property) in entity.properties().iter() {
                if let Ok(property) = property::Property::from_inputs(&format!("property__{}", key))
                {
                    properties.insert(key, property);
                } else {
                    properties.insert(key, current_property.clone());
                }
            }
            entity.set_properties(properties);
        }
    }
}
