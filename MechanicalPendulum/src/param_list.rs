// Parameter ID, value, tooltip
type Parameter = (String, f64, String);

pub struct ParamList {
    pub params: Vec<Parameter>,
}

impl ParamList {
    pub fn new() -> Self {
        Self {
            params: Vec::<Parameter>::new(),
        }
    }

    pub fn get(&self, id: usize) -> f64 {
        self.params[id].1
    }

    pub fn set(&mut self, id: usize, val: f64) {
        self.params[id].1 = val;
    }

    pub fn get_by_key(&self, key: &str) -> f64 {
        match self.params.iter().find(|&x| x.0.eq(key) ) {
        Some(x) => x.1,
        None => panic!("No parameter with key '{}' found", key),
        }
    }

    // pub fn set_by_key(&mut self, key: &str, val: f64) {
    //     match self.params.iter_mut().find(|x| (*x).0.eq(key) ) {
    //     Some(x) => x.1 = val,
    //     None => panic!("No parameter with key '{}' found", key),
    //     }
    // }

    pub fn get_key(&self, id: usize) -> String {
        self.params[id].0.clone()
    }

    pub fn get_tooltip(&self, id: usize) -> String {
        self.params[id].2.clone()
    }

    // pub fn iter(&self) -> std::slice::Iter<Parameter> {
    //     self.params.iter()
    // }

    // pub fn iter_mut(&mut self) -> std::slice::IterMut<Parameter> {
    //     self.params.iter_mut()
    // }

    pub fn len(&self) -> usize {
        self.params.len()
    }

    pub fn copy_from(&mut self, other: &ParamList) {
        self.params = other.params.clone();
    }
}

impl Clone for ParamList {
    fn clone(&self) -> Self {
        Self {
            params: self.params.clone(),
        }
    }

    fn clone_from(&mut self, other: &Self) {
        self.params.clone_from(&other.params);
    }
}

impl<const N: usize> From<[(&str, f64, &str); N]> for ParamList {
    fn from(arr: [(&str, f64, &str); N]) -> Self {
        let params = arr.iter().map(|i| {
            (i.0.to_string(), i.1, i.2.to_string() )
        }).collect();
        Self {
            params,
        }
    }
}

/// Trait of an object that can copy parameter list from some other location
pub trait Parametrized {
    fn copy_params_from(&mut self, other: &ParamList);
    fn get_params(&self) -> ParamList;
}
