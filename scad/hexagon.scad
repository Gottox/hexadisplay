count = 18;

seed=42+3+3+3+3+3+3+3+3+3+3+3+3+3+3+3+3+3+3+3+3+3+3+3;
random_height=rands(18,40,count,seed);
random_steep=rands(0,20,count,seed+1);
random_rotate=rands(0,360/6,count,seed+2);

//random_height=rands(0,0,count,seed);
//random_steep=rands(0,0,count,seed+1);
//random_rotate=rands(0,0,count,seed+2);

diameter = 50;
outer_radius = diameter / 2;
inner_radius = sqrt(pow(outer_radius,2) - pow(outer_radius/2,2));

/*!mirror([0,0,1]){
    translate([70,0,0]) edge_side(0);
    edge_side();
    translate([-70,0,0]) rotate([0,0,90]) edge_top(true);
    translate([-70,70,0]) rotate([0,0,90]) edge_top(false);
}
for(i = [0:count-1]) {
    translate([diameter * 1.3 * i,0,0]) {
        piece(random_height[i], random_steep[i], random_rotate[i]);
        //edge();
        //for(i=[0:2]) {
        //    translate([0,diameter - i * 8,0]) connector(-.2);
        //}
    }
}*/ 

corner_bottom();

%edge_top();

module corner_bottom(offset = 0, wall = 9) {
    difference() {
        cylinder(13, d = diameter, $fn = 6);
        
        translate([-diameter/2,-offset,0]) cube(diameter);
        translate([wall-outer_radius/2,-diameter/2,0]) cube(diameter);
    }
    
    rotate([0,0,-60]) translate([0,-inner_radius,3]) connector();
    
    translate([0,0,-13]) intersection() {
        cylinder(26, d = diameter, $fn = 6);
        union() {
            translate([-diameter/2,-offset,0]) cube([diameter/2,wall,26]);
            translate([-5,-diameter/2,0]) cube([5,diameter/2,26]);
        }
    }
    
}

module corner_top(offset = 0, wall = 9) {
    difference() {
        cylinder(13, d = diameter, $fn = 6);
        
        translate([-diameter/2,-offset,0]) cube(diameter);
        translate([wall-outer_radius/2,-diameter/2,0]) cube(diameter);
    }
    
    rotate([0,0,-60]) translate([0,-inner_radius,3]) connector();
    
    translate([0,0,-13]) intersection() {
        cylinder(26, d = diameter, $fn = 6);
        union() {
            translate([-diameter/2,-offset,0]) cube([diameter/2,wall,26]);
            translate([-5,-diameter/2,0]) cube([5,diameter/2,26]);
        }
    }
    
}

module edge_side(offset = inner_radius, wall = 9) {
    difference() {
        cylinder(13, d = diameter, $fn = 6);
        
        translate([-diameter/2,-offset,0]) cube(diameter);
    }
    
    translate([0,-inner_radius,3]) connector();
    
    translate([0,0,-13]) intersection() {
        cylinder(26, d = diameter, $fn = 6);
        translate([-diameter/2,-offset,0]) cube([diameter,wall,26]);
    }
}

module edge_top(edge = false,offset = outer_radius/2, wall = 9) {
    difference() {
        cylinder(13, d = diameter, $fn = 6);
        
        //piece_solid(2, 0, 0,diameter, 0, 1);
        translate([wall-offset,-diameter/2,0]) cube(diameter);
    }
    
    rotate([0,0,-120]) translate([0,-inner_radius,3]) connector();
    rotate([0,0,-60]) translate([0,-inner_radius,3]) connector();
    
    translate([0,0,-13]) intersection() {
        cylinder(26, d = diameter, $fn = 6);
        union() {
        translate([-5,-diameter/2,0]) cube([5,diameter,26]);
            //translate([0,-3,0]) cube([diameter,8,2]);
            if(edge) {
                #translate([-5-3,-diameter/2,0]) rotate([0,45,0]) cube([5,diameter,5]);
            }
        }
    }
}

module piece(height, steep, rotation) {
    rotate([0,0,rotation]) translate([-outer_radius,0,height + 10]) rotate([180,steep,0]) translate([outer_radius,0,0]) difference() {
        piece_solid(height, steep, rotation,diameter, 0);
        piece_solid(height-0.9, steep, rotation,diameter-2, 2);
    }
}

module piece_solid(height, steep, rotation, diameter, offset, out = 0) {
    difference() {
        rotate([0,0,rotation]) {
            difference() {
                cylinder(10+height+diameter, d=diameter,  $fn=6);
                for(i=[0:5]) rotate([0,0,i*60]) {
                    cube([15,100,6], center = true);
                    translate([0,inner_radius,0]) mirror([0,1,0]) connector(offset,10);
                }
            }
            if (out != 0) {
                for(i=[0:5]) rotate([0,0,i*60]) {
                    translate([0,inner_radius,0]) mirror([0,1,0]) connector(offset,10);
                }
            }
        }
        translate([-outer_radius,0,10+height]) rotate([0,steep,0]) translate([-diameter*2, -diameter*2, 0])cube(diameter*4);
    }
}

module connector(offset=0, height = 7) {
    z = height+offset;
    x = 3+offset/2;
    y = 5+offset;
    rotate = -45;
    for(m=[0,1]) mirror([0,m,0]) difference() {
        intersection() {
            translate([-y,0,0]) cube([y*2,x,z]);
            translate([0,0,z]) rotate([-90,0,0]) linear_extrude(height = x, scale = 2) square([y, z*2], center = true); 
        }
        translate([-outer_radius,0,z]) rotate([rotate,0,0])  cube(outer_radius*2);
        //cube([3,100,6], center = true);
    }
    
}