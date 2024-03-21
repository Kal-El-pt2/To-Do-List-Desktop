slint::slint! {
    import { VerticalBox, CheckBox, LineEdit, Button, HorizontalBox } from "std-widgets.slint";
  
    export component ListItem {
      width: 280px;
      height: 50px;
  
      HorizontalBox {
        CheckBox {
          checked: false;
          text: "";
        }
        LineEdit {
          text: "Task Name";
        }
      }
    }
    struct ListMember{
      checked:bool
    }
    
    export component List {
      in-out property <int> itemCount: 2;
      callback Increment();
      in property <[ListMember]> list_members: [
        {checked:false},
        {checked:false},
      ];
  
      VerticalBox {
        for lm[i] in list_members : ListItem {
            width: 100px;
            height: 64px;
        }
      }
    }
  
    export component AddButton {
      callback AddListItem();
  
      btn := Button {
        text: "+Add task";
        
      }
    }
  
    export component ScreenView {
      width: 300px;
      height: 300px;
  
      VerticalBox {
        spacing: 5px;
        padding: 5px;
        alignment: center;
  
        list:= List {
          Increment => {
            self.itemCount=self.itemCount+1;
          }
        }
        btn:=AddButton {
          AddListItem=>{
            list.Increment();
          }
        }
      }
    }
  }
  
  slint::include_modules!();
  
  fn main() {
    let screen = ScreenView::new().unwrap(); // Declare and unwrap ScreenView
    screen.run().unwrap(); // Call methods on the declared variable
  }
  