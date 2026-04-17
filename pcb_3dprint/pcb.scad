difference() {
    union() {
        translate([-5,0,0]) cube([50,50,1]);
        linear_extrude(1.8) import("rusty-bc250-atx-F_Cu_route.svg");
    }
    translate([0,0,-0.5]) linear_extrude(3) import("rusty-bc250-atx-F_Cu_drill.svg");
    
    translate([-2,3,-3]) cylinder(h=6, r=1.6, $fn=20);
    translate([-2+44,3,-3]) cylinder(h=6, r=1.6, $fn=20);
    #translate([-2+44,0+47,-3]) cylinder(h=6, r=1.6, $fn=20);
    translate([-2,0+47,-3]) cylinder(h=6, r=1.6, $fn=20);
}