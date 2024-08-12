// realityscript for ai3d co-create version: 0.1.0

$(add_objects).set({"sphere","cube","cylinder"}).layout({"bottom_left_vertical"});

$(ar).use("arcore","arkit").define("instantaneous_position":"camera.main.transform.position","instantaneous_rotation":"camera.main.transform.rotation");

$(easel).use("lcm").add("slider":"lcm_image","slider":"clarity_ai","prompt_refine","gemini");

$(button_cam).onClick(function(instantaneous_position,instantaneous_rotation){

    $(easel).instantiate(instantaneous_position,instantaneous_rotation);

});