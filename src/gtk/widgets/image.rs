// This file is part of rgtk.
//
// rgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with rgtk.  If not, see <http://www.gnu.org/licenses/>.

//! A widget displaying an image

use gtk::ffi;
use gtk::traits;

/// Image — A widget displaying an image
struct_Widget!(Image)


impl Image {
    pub fn new_from_file(filename: &str) -> Option<Image> {
        let tmp_pointer = unsafe {
            filename.with_c_str(|c_str| {
                ffi::gtk_image_new_from_file(c_str)
            })
        };
        check_pointer!(tmp_pointer, Image)
    }
}

impl_drop!(Image)
impl_TraitWidget!(Image)

impl traits::Misc for Image {}
