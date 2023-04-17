use slint;

slint::slint! {
    import {LineEdit, TextEdit, ComboBox, GridBox, VerticalBox, HorizontalBox, StyleMetrics} from "std-widgets.slint";

    export component HelloWorld inherits Window {
        preferred-width: 800px;
        preferred-height: 800px;

        HorizontalLayout {
            // Search and chat groups
            VerticalLayout {
                HorizontalLayout {
                    min-width: 70px;
                    height: 20px;

                    TouchArea {
                        Text {
                            font-size: 20px;
                            text: "Menu";
                            color: gray;
                        }
                    }
                    LineEdit {
                        enabled: true;
                        font-size: 20px;
                        placeholder-text: "Search";
                    }
                }
                Text {
                    text: "Hello, world";
                    color: blue;
                }
                Text {
                    text: "Hello, world";
                    color: blue;
                }
            }
            Text {
                text: "Hello, world";
                color: blue;
            }
            Text {
                text: "Hello, world";
                color: blue;
            }
        }

    }
}

fn main() {
    HelloWorld::new().unwrap().run().unwrap();
}