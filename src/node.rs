#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub size: String,
    pub count: u32,
}

#[derive(Debug)]
pub struct KubernetesCluster {
    name: String,
    version: String,
    auto_upgrade: bool,
    node_pool: Option<Vec<Node>>,
}

impl KubernetesCluster {
    pub fn new(name: String, version: String) -> KubernetesClusterBuilder {
        KubernetesClusterBuilder {
            name,
            version,
            auto_upgrade: None,
            node_pool: None,
        }
    }
}

pub struct KubernetesClusterBuilder {
    name: String,
    version: String,
    auto_upgrade: Option<bool>,
    node_pool: Option<Vec<Node>>,
}

impl KubernetesClusterBuilder {
    pub fn auto_upgrade(&mut self, auto_upgrade: bool) -> &mut Self {
        self.auto_upgrade = Some(auto_upgrade);
        self
    }

    pub fn node_pool(&mut self, node_pool: Vec<Node>) -> &mut Self {
        self.node_pool = Some(node_pool);
        self
    }

    pub fn build(&self) -> KubernetesCluster {
        KubernetesCluster {
            name: self.name.clone(),
            version: self.version.clone(),
            auto_upgrade: self.auto_upgrade.unwrap_or(false),
            node_pool: self.node_pool.clone(),
        }
    }
}
