#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod classcontract {
    use ink_prelude::string::String;
    use ink_storage::{
        traits::{PackedLayout, SpreadAllocate, SpreadLayout},
        Mapping,
    };
    use scale::{Decode, Encode};

    type Id = u32;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Classcontract {
        teacher: AccountId,
        student_name: Mapping<Id, String>,
        student_level: Mapping<Id, Level>,
    }

    #[derive(Debug, SpreadLayout, PackedLayout, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Level {
        Excellent,
        Good,
        Average,
        Fail,
        NotUpdated,
    }

    #[ink(event)]
    pub struct UpdateStudent {
        student: Option<String>,
        point: Option<u32>,
    }

    impl Classcontract {
        /// Khoi tao contract de cap nhat xep loai hoc luc cua hoc sinh
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                let owner = Self::env().caller();
                contract.teacher = owner;
                contract.student_name = Default::default();
                contract.student_level = Default::default();
            })
        }

        /// Khoi tao contract de cap nhat xep loai hoc luc cua hoc sinh
        #[ink(constructor)]
        pub fn default() -> Self {
            ink_lang::utils::initialize_contract(|_| {})
        }

        /// Cap nhat thong tin cua hoc sinh: ma so hoc sinh, ten va diem tong ket nam
        #[ink(message)]
        pub fn update_student(&mut self, id: u32, name: String, point: u32) {
            let caller = self.env().caller();
            assert!(
                caller == self.teacher,
                "Ban khong phai la giao vien chu nhiem!!!"
            );
            assert!(
                point <= 10 && point > 0,
                "Diem tong ket khong hop le!!! Hay thu lai."
            );
            let level = match point {
                x if x >= 8 && x <= 10 => Level::Excellent,
                x if x >= 7 && x < 8 => Level::Good,
                x if x >= 5 && x < 7 => Level::Average,
                _ => Level::Fail,
            };
            self.student_name.insert(&id, &name);
            self.student_level.insert(&id, &level);
            self.env().emit_event(UpdateStudent {
                student: Some(name),
                point: Some(point),
            });
        }

        /// Tra ve thong tin ten cua hoc sinh thong qua ma so hoc sinh
        #[ink(message)]
        pub fn get_student_name(&self, id: u32) -> String {
            self.student_name.get(&id).unwrap_or_default()
        }

        /// Tra ve xep loai cua hoc sinh thong qua ma so hoc sinh
        #[ink(message)]
        pub fn get_student_level(&self, id: u32) -> Level {
            self.student_level.get(&id).unwrap_or(Level::NotUpdated)
        }
    }

    //Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    //module and test functions are marked with a `#[test]` attribute.
    //The below code is technically just normal Rust code.
    // #[cfg(test)]
    // mod tests {
    //     /// Imports all the definitions from the outer scope so we can use them here.
    //     use super::*;

    //     /// Imports `ink_lang` so we can use `#[ink::test]`.
    //     use ink_lang as ink;

    //     /// We test if the default constructor does its job.
    //     #[ink::test]
    //     fn default_works() {
    //         let classcontract = Classcontract::default();
    //         assert_eq!(classcontract.get(), false);
    //     }

    //     /// We test a simple use case of our contract.
    //     #[ink::test]
    //     fn it_works() {
    //         let mut classcontract = Classcontract::new(false);
    //         assert_eq!(classcontract.get(), false);
    //         classcontract.flip();
    //         assert_eq!(classcontract.get(), true);
    //     }
    // }
}
