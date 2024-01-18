use node::{KubernetesCluster, Node};

mod builders;
mod cars;
mod components;
mod director;
mod foo;
mod node;

use builders::{Builder, CarBuilder, CarManualBuilder};
use cars::{Car, Manual};
use director::Director;

fn main() {
    let name = "my-cluster".to_owned();
    let version = "1.25.0".to_owned();

    let nodes = vec![Node {
        name: "node-1".to_owned(),
        size: "small".to_owned(),
        count: 1,
    }];

    let basic_cluster = KubernetesCluster::new(name.clone(), version.clone()).build();

    let auto_upgrade_cluster = KubernetesCluster::new(name.clone(), version.clone())
        .auto_upgrade(true)
        .build();

    let complete_cluster = KubernetesCluster::new(name.clone(), version.clone())
        .auto_upgrade(true)
        .node_pool(nodes)
        .build();

    println!("{:#?}", basic_cluster);
    println!("======");
    println!("{:#?}", auto_upgrade_cluster);
    println!("======");
    println!("{:#?}", complete_cluster);

    println!("======");
    let fb = foo::FooBuilder::new().name(String::from("Y")).build();
    println!("{:#?}", fb);

    println!("======");
    let mut car_builder = CarBuilder::default();

    // Director gets the concrete builder object from the client
    // (application code). That's because application knows better which
    // builder to use to get a specific product.
    Director::construct_sports_car(&mut car_builder);

    // The final product is often retrieved from a builder object, since
    // Director is not aware and not dependent on concrete builders and
    // products.
    let car: Car = car_builder.build();
    println!("Car built: {:?}\n", car.car_type());

    let mut manual_builder = CarManualBuilder::default();

    // Director may know several building recipes.
    Director::construct_city_car(&mut manual_builder);

    // The final car manual.
    let manual: Manual = manual_builder.build();
    println!("Car manual built:\n{}", manual);
}
