
use egui_plot::{Line, Plot, PlotPoints};
use std::time::Duration;

use egui;
use egui_plotter::EguiBackend;
use plotters::prelude::*;
use std::ops::Range;
use roots::find_roots_quartic;
const M: f64 = 1.0;
const A: f64 = 0.999;
const E:f64 = 0.8889917687523382;
const LZ:f64 =  1.92511617871112;
const C: f64 = 1.2876760482236116;
const dt :f64 = 0.01;


fn get_theta_poly_coefficients( lz:f64, e:f64, c:f64)->[f64;5]{ // coefficients for 0th, cos()^2 and cos()^4
    let a0 = c;
    let a2 = -(c+A.powi(2)*(1.0-e.powi(2))+lz.powi(2));
    let a4 = A.powi(2)*(1.0-e.powi(2));
    [a0,0.0,a2,0.0,a4]
}
fn get_radial_poly_coefficients( lz:f64, e:f64, c:f64)->[f64;5]{
    let a4 = e.powi(2)-1.0;
    let a3 = 2.0*M;
    let a2 = e.powi(2)*2.0*A.powi(2)-2.0*A*lz*e-(A*e-lz).powi(2)-c-A.powi(2);
    let a1 = 2.0*M*(A*e-lz).powi(2)+2.0*M*c;
    let a0 = e.powi(2)*A.powi(4)-2.0*A*lz*e*A.powi(2)+A.powi(2)*lz.powi(2)-A.powi(2)*(A*e-lz).powi(2)-A.powi(2)*c;
  //  println!("{:?}",[a0,a1,a2,a3,a4]);
    [a0,a1,a2,a3,a4]
}

fn r_derivative(x:f64, coeffs:[f64;5]) ->f64{
    let mut sum = 0.0;
    for i in (0..coeffs.len()){
        sum += coeffs[i]*x.powi(i as i32)
    }
    sum.abs().sqrt()
}

fn calculate_poly_derivative_theta_abs_sqrt(x:f64, coeffs:[f64;5])->f64{
    let c = x.cos();
    let mut sum = 0.0;
    for i in (0..coeffs.len()){
        sum += coeffs[i]*c.powi(i as i32)
    }
    sum = sum.abs().sqrt()/x.sin();
    sum

}

fn integrate(y_min:f64, y_max:f64, coefficient_calculator: &dyn Fn(f64,f64,f64) -> [f64;5], derivative:  &dyn Fn(f64, [f64;5])->f64, lz:f64, e:f64, c:f64) -> PlotPoints{ //function should
    let coefficients = coefficient_calculator( lz, e, c);
    let mut going_up:bool = true;
    let mut last_switch_location:f64 = 0.0;
    let mut y = y_min;

    (0..20000).map(|i| {
        let x = i as f64 * dt;
        let increment = {if going_up {derivative(y,coefficients)} else {-derivative(y,coefficients)}}*dt;
        y += increment;
        if ((y-y_min).abs() < 0.001 || (y-y_max).abs() < 0.001) && (x - last_switch_location).abs() > 1.0{
            going_up = !going_up;
            last_switch_location = x;
          //  println!("Switched at {}, going up is {}",last_switch_location,going_up)
        }
        [x, y]
    }).collect()
}

fn find_phi(r_plot_points: PlotPoints, t_plot_points: PlotPoints,   lz:f64, e:f64) { //->PlotPoints
    let r_points = r_plot_points.points().to_vec();
    let t_points = t_plot_points.points().to_vec();
    let traj: Vec<[f64; 2]> = (0..20000).map(|i| [r_points[i].y,t_points[i].y] ).collect();

    let mut phi_vec: Vec<[f64; 2]> = Vec::new();

    for point in traj{
        let r = point[0];
        let theta = point[1];
       // phi_vec.append(r_points)
        let phi = phi_total(theta,r,lz,e);

    }

   // println!("{:?}",y[0]);
  //  for point in y.points(){
 //       println!("{:?}, {:?}",point.x, point.y);
  //  }

}


#[derive(Default)]

struct Graph {}


impl eframe::App for Graph {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.5);
        egui::CentralPanel::default().show(ctx, |ui| {

            let root = EguiBackend::new(ui).into_drawing_area();

            root.fill(&WHITE).unwrap();

            let x_axis = (-3.0..3.0).step(0.1);
            let z_axis = (-3.0..3.0).step(0.1);

            let mut chart = ChartBuilder::on(&root)
                .caption(format!("3D Plot Test"), (FontFamily::SansSerif, 20))
                .build_cartesian_3d(x_axis, -3.0..3.0, z_axis)
                .unwrap();


            chart
                .configure_axes()
                .light_grid_style(BLACK.mix(0.15))
                .max_light_lines(3)
                .draw()
                .unwrap();


            chart
                .draw_series(LineSeries::new(
                    (-100..100)
                        .map(|y| y as f64 / 40.0)
                        .map(|y| ((y * 10.0).sin(), y, (y * 10.0).cos())),
                    &BLACK,
                ))
                .unwrap()
                .label("Line")
                .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLACK));


            root.present().unwrap();
        });




        //  egui::CentralPanel::default().show(ctx, |ui| {

    //        ui.heading("In Mino Time");


     //       let radial = integrate(2.0, 6.0, & get_radial_poly_coefficients, &r_derivative, LZ, E, C);
     //       let angular = integrate(1.0471975511965979,2.094395102365872, & get_theta_poly_coefficients, & calculate_poly_derivative_theta_abs_sqrt,LZ,E,C);
     //       find_phi( radial, angular,LZ, E);
           // let radial_line = Line::new(radial);
          //  let angular_line = Line::new(angular);


           // Plot::new("my_plot").view_aspect(2.0).show(ui, |plot_ui| );  //{plot_ui.line(radial_line); plot_ui.line(angular_line)}



        //    if ui.button("Quit").clicked() {
        //        std::process::exit(0);
       //     };
      //  });

        egui::SidePanel::right("side pannel").show(
            ctx,
            |ui| {
                ui.heading("R ");

                ui.label("This is a ui.label");
                let coeffs = get_radial_poly_coefficients(LZ,E,C);


                let R: PlotPoints = (0..1000).map(|i| {
                    let x = i as f64 * 0.01;

                    [x, r_derivative(x, coeffs)-radial(x, LZ, E, C)]
                   // [x,x]

                }).collect();

                let line = Line::new(R);
                let R2: PlotPoints = (0..1000).map(|i| {
                    let x = i as f64 * 0.01;

                    //  [x, Theta(x,LZ,E,C)]
                    [x,x]

                }).collect();

                let line2 = Line::new(R2);



                Plot::new("my_plot").view_aspect(2.0).show(ui, |plot_ui| {plot_ui.line(line); });

                // This literally creates the button AND checks to see if it was clicked
                if ui.button("Quit").clicked() {
                    std::process::exit(0);
                };
            }
        );
    }
}

fn main() -> eframe::Result<()> {
  //  println!("{}",radial(2.0,LZ,E,C));


    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size((800.0, 800.0)),
        ..eframe::NativeOptions::default()
    };

    eframe::run_native(
        "Visulizator",
        native_options,
        Box::new(|_| Ok(Box::<Graph>::default())),
    )

}



fn delta(r:f64) -> f64 {
    r.powi(2) - 2.0 * M * r + A.powi(2)
}
fn sigma(r:f64, c:f64) -> f64 {
    r.powi(2) + A.powi(2)*c.powi(2)
}
fn p(r:f64, lz:f64,e:f64) -> f64 {
    e*(r.powi(2) + A.powi(2))-A*lz
}
fn radial(r:f64, lz:f64, e:f64, c:f64) -> f64{
    (p(r,lz,e).powi(2)-delta(r)*(r.powi(2) + (A*e-lz).powi(2)+c))

    //   (p(r,lz,e).powi(2)-delta(r)*(r.powi(2) + (A*e-lz).powi(2)+c))
}

fn phi_r(r:f64, lz:f64,e:f64) -> f64{
    let d = delta(r);
    let p = p(r,lz,e);
    A*p/d
}

fn phi_theta(theta:f64,lz:f64) -> f64 {
    lz/(1.0-theta.cos().powi(2))
}

fn phi_total(theta:f64,r:f64, lz:f64,e:f64) -> f64 {
    phi_r(r,lz,e) + phi_theta(theta,lz) -A*e
}


