import dearpygui.dearpygui as imgui

def main():
    imgui.create_context()
    imgui.create_viewport()
    imgui.setup_dearpygui()
    imgui.show_viewport()
    imgui.start_dearpygui()

if __name__ == "__main__":
    main()
