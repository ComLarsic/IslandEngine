
SPEED = 10

def init(ctx, scene):
    # Send 'Hello, script!' to the logger
    ctx.log_info("Hello, script!")
    return scene

def update(ctx, scene):
    # Control the entity if is has a position and a scale component
    for entity in scene["entities"]:
        if "position" in entity and "scale" in entity: 
            if ctx.is_key_down("W"):
                entity["position"]["y"] -= SPEED
            elif ctx.is_key_down("S"):
                entity["position"]["y"] += SPEED

            if ctx.is_key_down("A"):
                entity["position"]["x"] -= SPEED
            elif ctx.is_key_down("D"):
                entity["position"]["x"] += SPEED
    
    # Camera movement
    if ctx.is_key_down("Up"):
        scene["camera"]["position"]["y"] -= 10
    elif ctx.is_key_down("Down"):
        scene["camera"]["position"]["y"] += 10

    if ctx.is_key_down("Left"):
        scene["camera"]["position"]["x"] -= 10
    if ctx.is_key_down("Right"):
        scene["camera"]["position"]["x"] += 10
    
    return scene