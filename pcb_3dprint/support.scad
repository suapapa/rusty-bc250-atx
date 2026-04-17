difference() {
    /*
    union() {
        translate([-5,0,0]) cube([50,50,1]);
        linear_extrude(1.8) import("rusty-bc250-atx-F_Cu_route.svg");
    }
    translate([0,0,-0.5]) linear_extrude(3) import("rusty-bc250-atx-F_Cu_drill.svg");
    */
    

    difference() {    
        union() {
            translate([-5,0,0]) cube([50,50,1]);
            translate([-2,3,0]) cylinder(h=5, r=3, $fn=20);
            translate([-2+44,3,0]) cylinder(h=5, r=3, $fn=20);
            #translate([-2+44,0+47,0]) cylinder(h=5, r=3, $fn=20);
            translate([-2,0+47,0]) cylinder(h=5, r=3, $fn=20);
        }
        translate([-2,3,-3]) cylinder(h=16, r=1.6, $fn=20);
        translate([-2+44,3,-3]) cylinder(h=16, r=1.6, $fn=20);
        #translate([-2+44,0+47,-3]) cylinder(h=16, r=1.6, $fn=20);
        translate([-2,0+47,-3]) cylinder(h=16, r=1.6, $fn=20);
    }
}