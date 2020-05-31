const grav:f32 = 0.5f32;
use super::Input;
enum sumoState{
    Neutral,
    Jump,
    Dodge,
    Charge
}

pub struct Sumo{
    posX: f32,
    posY: f32,
    velY: f32,
    velX: f32,
    height: i32,
    width: i32,
    state: sumoState
}
impl Sumo{
    pub fn new(x: f32,y: f32, height: i32, width: i32) -> Sumo{
        Sumo{
            posX: x,
            posY: y,
            velX: 0f32,
            velY: 0f32,
            height: height,
            width: width,
            state: sumoState::Neutral
        }
    }
}
pub struct Floor{
    posX: f32,
    posY: f32,
    height: i32,
    width: i32,
}


pub struct Game{
    leftSumo: Sumo,
    rightSumo: Sumo,
    floor: Floor,
    leftScore: i32,
    rightScore: i32,
    currentTime: std::time::SystemTime,
    accumulator: std::time::Duration

}
impl Game{
    pub fn update(&self,leftActions: &[Input] ,rightActions: &[Input]) {
        let updateTime = self.currentTime.elapsed().unwrap();
        self.currentTime = std::time::SystemTime::now();
        self.accumulator+= updateTime;
        let mut sumos = [self.leftSumo,self.rightSumo];
        
        

        while self.accumulator >= super::TIME_STEP {
            
            for sumo in sumos.iter_mut(){
                match sumo.state {
                    sumoState::Jump =>{sumo.velY-= grav;}
                    sumoState::Charge => {
                        if sumo.velX < -1.0 {
                            sumo.velX+=1.0;
                        }else if sumo.velX < 0.0 {
                            sumo.velX=0.0;
                            sumo.state=sumoState::Neutral;
                        }else if sumo.velX > 1.0 {
                            sumo.velX-=1.0;
                        }else if sumo.velX < 1.0 {
                            sumo.velX=0.0;
                            sumo.state=sumoState::Neutral;
                        }else{
                            sumo.state=sumoState::Neutral;
                        }
                    }


                }


                //Check if sumos go under the floor if they do stop them.
                if sumo.posY < 50f32 {
                    sumo.state = sumoState::Neutral;
                    sumo.posY = 50f32;
                }


            }
            for action in leftActions{
                match action {
                    
                }
            }

            self.accumulator -= super::TIME_STEP;
        }
    }
}

pub fn init_game() -> Game{
    Game{
        leftSumo: Sumo::new(-250f32,50f32,100,50),
        rightSumo: Sumo::new(250f32,50f32,100,50),
        floor: Floor{posX:0f32,posY:0f32,height:50,width:1000},
        leftScore:0,
        rightScore:0,
        accumulator: std::time::Duration::new(0,0),
        currentTime: std::time::SystemTime::now()
    }
}