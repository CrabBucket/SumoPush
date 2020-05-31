const grav:f32 = 0.5f32;
use super::Input;
//Sumo state is the possible state our sumo character can be in
enum SumoState{
    Neutral,
    Jump,
    Dodge,
    Charge
}

//Our sumo characters that are fighting.
pub struct Sumo{
    posX: f32,
    posY: f32,
    velY: f32,
    velX: f32,
    height: i32,
    width: i32,
    state: SumoState
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
            state: SumoState::Neutral
        }
    }
}
//The floor that the sumo characters stand on
pub struct Floor{
    posX: f32,
    posY: f32,
    height: i32,
    width: i32,
}

//The gamestate that the server tracks
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
    //Updates the gamestate based on the actions that left and right players take.
    pub fn update(mut self,leftActions: &[Input] ,rightActions: &[Input]) {
        //Update time is the time since the last time we updated the simulation
        let updateTime = self.currentTime.elapsed().unwrap();
        //Current time is the time we started updating this simulation
        self.currentTime = std::time::SystemTime::now();
        //Accumulator is incremented by the amount of time it took us to update out last frame 
        //(we only want to update every so often but if we miss updates due to lag we want to quickly fix our state)
        self.accumulator+= updateTime;
        //Convient way to store both the left and right sumo to iterate
        let mut sumos = [self.leftSumo,self.rightSumo];
        
        
        // this updates the simulation while we have accumulated time still left.
        // AS long as the accumulator is greater than TIME_STEP (our server tickrate essentially) we continue to fix the state till we catch up.
        while self.accumulator >= super::TIME_STEP {
            //This for loop handles the non-player input update portion of the gamestate such as physics that must happen regardless of player input (collisions, gravity, etc)
            for sumo in sumos.iter_mut(){
                match sumo.state {
                    //If the sumo is in the jump state we want them to update their y velocity based on gravity
                    SumoState::Jump =>{sumo.velY-= grav;}
                    //IF they are charging we slow them down gradually until the reach zero and set them to to neutral
                    SumoState::Charge => {
                        if sumo.velX < -1.0 {
                            sumo.velX+=1.0;
                        }else if sumo.velX < 0.0 {
                            sumo.velX=0.0;
                            sumo.state=SumoState::Neutral;
                        }else if sumo.velX > 1.0 {
                            sumo.velX-=1.0;
                        }else if sumo.velX < 1.0 {
                            sumo.velX=0.0;
                            sumo.state=SumoState::Neutral;
                        }else{
                            sumo.state=SumoState::Neutral;
                        }
                    }


                }


                //Check if sumos go under the floor if they do stop them.
                if sumo.posY < 50f32 {
                    sumo.state = SumoState::Neutral;
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