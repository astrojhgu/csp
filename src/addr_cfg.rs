use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, PartialEq, Eq)]
pub struct FiberAddrPair{
    pub srv_addr: [u16;4],
    pub srv_port: u16,
    pub dev_addr: [u16;4],
    pub dev_port: u16,
}


#[derive(Deserialize, Serialize, Debug, Default, PartialEq, Eq)]
pub struct AddrCfg{
    pub beam1: [FiberAddrPair; 4], 
    pub beam2: [FiberAddrPair; 4],
}

impl AddrCfg{
    pub fn to_raw(&self)->Vec<u8>{
        let mut result=vec![0_u8; 160];

        let raw_data=unsafe{std::slice::from_raw_parts_mut(result.as_mut_ptr() as *mut u16, 80)};

        for (i, a) in self.beam1.iter().enumerate(){
            for j in 0..4{
                raw_data[i*10+j]=a.srv_addr[j];
                raw_data[i*10+5+j]=a.dev_addr[j];
            }
            raw_data[i*10+4]=a.srv_port;
            raw_data[i*10+9]=a.dev_port;
        }

        for (i, a) in self.beam2.iter().enumerate(){
            for j in 0..4{
                raw_data[40+i*10+j]=a.srv_addr[j];
                raw_data[40+i*10+5+j]=a.dev_addr[j];
            }
            raw_data[40+i*10+4]=a.srv_port;
            raw_data[40+i*10+9]=a.dev_port;
        }
        result
    }

    pub fn from_raw(input: &[u8])->AddrCfg{
        assert_eq!(input.len(), 160);
        let mut result=Self::default();
        let raw_data=unsafe{std::slice::from_raw_parts(input.as_ptr() as *const u16, 80)};

        for (i, a) in result.beam1.iter_mut().enumerate(){
            for j in 0..4{
                a.srv_addr[j]=raw_data[i*10+j];
                a.dev_addr[j]=raw_data[i*10+5+j];
            }
            a.srv_port=raw_data[i*10+4];
            a.dev_port=raw_data[i*10+9];
        }

        for (i, a) in result.beam2.iter_mut().enumerate(){
            for j in 0..4{
                a.srv_addr[j]=raw_data[40+i*10+j];
                a.dev_addr[j]=raw_data[40+i*10+5+j];
            }
            a.srv_port=raw_data[40+i*10+4];
            a.dev_port=raw_data[40+i*10+9];
        }

        result
    }
}
