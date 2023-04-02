from node import node_tool


@node_tool
def is_tip_available(input: str):
    """
    mode: condition
    description: Verify if a tip is available for use
    preconditions: The pipette is attached to the system and selected as the active tool
    """
    return "Tip is available"


@node_tool
def tip_stock_error(input: str):
    """
    mode: action
    description: Address an error related to the tip stock
    preconditions: None 
    """
    return "Tip stock error"


@node_tool
def is_tip_available_in_tray(input: str):
    """
    mode: condition
    description: Check if the tip is available in the tray
    preconditions: Pipette is present, Pipette is the actively mounted tool
    """
    return "Tip is available in tray"


@node_tool
def is_discard_success(input: str):
    """
    mode: condition
    description: Check if the discard was successful
    preconditions: Pipette is present, Pipette is the actively mounted tool
    """
    return "Discard was successful"


@node_tool
def tray_maintainance_error(input: str):
    """
    mode: action
    description: Handle tray maintainance error
    preconditions: Pipette is present, Pipette is the actively mounted tool
    """
    return "Tray maintainance error"


@node_tool
def is_load_new_tray_successful(input: str):
    """
    mode: condition
    description: Check if the load new tray was successful
    preconditions: Pipette is present, Pipette is the actively mounted tool
    """
    return "Load new tray was successful"


@node_tool
def load_new_tray_maintenance_error(input: str):
    """
    mode: action
    description: Handle load new tray maintainance error
    preconditions: Pipette is present, Pipette is the actively mounted tool
    """
    return "Load new tray maintainance error"


@node_tool
def is_already_in_position(input: str):
    """
    mode: condition
    description: Check if the tip slider is already in position
    preconditions: Pipette is present, Pipette is the actively mounted tool
    """
    return "Tip slider is already in position"


@node_tool
def is_slider_position_reached(input: str):
    """
    mode: condition
    description: Check if the tip slider position was reached
    preconditions: Pipette is present, Pipette is the actively mounted tool
    """
    return "Tip slider position was reached"


@node_tool
def is_caught_tip_firm_and_oriented(input: str):
    """
    mode: condition
    description: Check if the caught tip is firm and oriented
    preconditions: Pipette is present, Pipette is the actively mounted tool
    """
    return "Caught tip is firm and oriented"


@node_tool
def is_pick_up_success(input: str):
    """
    mode: condition
    description: Check if the pick up was successful
    preconditions: Pipette is present, Pipette is the actively mounted tool
    """
    return "Pick up was successful"


@node_tool
def handle_pickup_failure(input: str):
    """
    mode: action
    description: Handle pick up failure
    preconditions: Pipette is present, Pipette is the actively mounted tool
    """
    return "Pick up failure"


@node_tool
def move_slider_to_load_position(input: str):
    """
    mode: action
    description: Move the tip slider to load position
    preconditions: Pipette is present, Pipette is the actively mounted tool
    """
    return "Moved tip slider to load position"


@node_tool
def load_next_tray(input: str):
    """
    mode: action
    description: Load the next tray
    preconditions: Pipette is present, Pipette is the actively mounted tool
    """
    return "Loaded next tray"


@node_tool
def move_tip_slider_to_position(input: str):
    """
    mode: action
    description: Move the tip slider to position
    preconditions: Pipette is present, Pipette is the actively mounted tool
    """
    return "Moved tip slider to position"


@node_tool
def pick_up_tip_using_gantry(input: str):
    """
    mode: action
    description: Pick up tip using gantry
    preconditions: Pipette is present, Pipette is the actively mounted tool
    """
    return "Picked up tip using gantry"


@node_tool
def goto_discard_position(input: str):
    """
    mode: action
    description: Go to discard position
    preconditions: Pipette is present, Pipette is the actively mounted tool
    """
    return "Went to discard position"


@node_tool
def prepare_to_discard(input: str):
    """
    mode: action
    description: Prepare to discard
    preconditions: Pipette is present, Pipette is the actively mounted tool
    """
    return "Prepared to discard"


@node_tool
def eject_tip(input: str):
    """
    mode: action
    description: Eject tip
    preconditions: Pipette is present, Pipette is the actively mounted tool
    """
    return "Ejected tip"


@node_tool
def is_discard_tip_successful(input: str):
    """
    mode: condition
    description: Check if the discard tip was successful
    preconditions: Pipette is present, Pipette is the actively mounted tool
    """
    return "Discard tip was successful"
