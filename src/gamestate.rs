const grav:i32 = 1;

use super::Input;
//Sumo state is the possible state our sumo character can be in
#[derive(PartialEq)]
enum SumoState{
    Neutral = 0,
    Jump = 1,
    Dodge = 2, 
    Charge = 3
}

//Our sumo characters that are fighting.
pub struct Sumo{
    posX: i32,
    posY: i32,
    velY: i32,
    velX: i32,
    height: i32,
    width: i32,
    state: SumoState,
    dodgestart: std::time::SystemTime
}
impl Sumo{
    pub fn new(x: i32,y: i32, height: i32, width: i32) -> Sumo{
        Sumo{
            posX: x,
            posY: y,
            velX: 0,
            velY: 0,
            height: height,
            width: width,
            state: SumoState::Neutral,
            dodgestart: std::time::SystemTime::now()
        }
    }
}
//The floor that the sumo characters stand on
pub struct Floor{
    posX: i32,
    posY: i32,
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
                if sumo.posY < 50 {
                    sumo.state = SumoState::Neutral;
                    sumo.posY = 50;
                }
                match sumo.state {
                    //If the sumo is in the jump state we want them to update their y velocity based on gravity
                    SumoState::Jump =>{sumo.velY-= grav;}
                    //IF they are charging we slow them down gradually until the reach zero and set them to to neutral
                    SumoState::Charge => {
                        let chargefactor = 3;
                        if sumo.velX < -chargefactor{
                            sumo.velX+=chargefactor;
                        }else if sumo.velX < 0{
                            sumo.velX=0;
                            sumo.state=SumoState::Neutral;
                        }else if sumo.velX > chargefactor{
                            sumo.velX-=chargefactor;
                        }else if sumo.velX < chargefactor{
                            sumo.velX=0;
                            sumo.state=SumoState::Neutral;
                        }else{
                            sumo.state=SumoState::Neutral;
                        }
                        continue;
                    }
                    SumoState::Dodge => {
                        if sumo.dodgestart.elapsed().unwrap() > std::time::Duration::from_millis(500) {
                            sumo.state = SumoState::Neutral;
                        }else{
                            continue;
                        }
                        
                    }
                    SumoState::Neutral => {

                    }
                    


                }
                sumo.posX += sumo.velX;
                sumo.posY += sumo.velY;
                if sumo.state != SumoState::Jump {
                    sumo.velX -= 1
                }


                //Check if sumos go under the floor if they do stop them.
                


            }
            fn action_tree (input: &Input, sumo: &mut Sumo){
                let movefactor = 2;
                match input {
                    
                    Input::Left => {
                        sumo.velX -= movefactor;
                    }
                    Input::Right => {
                        sumo.velX += movefactor;
                    }
                    Input::Charge => {
                        if sumo.state == SumoState::Charge {return;}
                        if sumo.velX > 0 {
                            sumo.velX += 30;
                        }else if sumo.velX < 0  {
                            sumo.velX -= 30;
                        }
                        sumo.state = SumoState::Charge;
                    }
                    Input::Dodge => {
                        if sumo.state != SumoState::Neutral {
                            return;
                        }
                        sumo.dodgestart = std::time::SystemTime::now();
                        sumo.state = SumoState::Dodge;
                    }
                    Input::Jump => {
                        if sumo.state != SumoState::Neutral {
                            return;
                        }
                        sumo.velY += 30;
                        sumo.state = SumoState::Jump;
                    }
                }
            }
            for action in leftActions{
                action_tree(action, &mut sumos[0]);
            }
            for action in rightActions{
                action_tree(action, &mut sumos[1]);
            }

            self.accumulator -= super::TIME_STEP;
        }
    }
}

pub fn init_game() -> Game{
    Game{
        leftSumo: Sumo::new(-250,50,100,50),
        rightSumo: Sumo::new(250,50,100,50),
        floor: Floor{posX:0,posY:0,height:50,width:1000},
        leftScore:0,
        rightScore:0,
        accumulator: std::time::Duration::new(0,0),
        currentTime: std::time::SystemTime::now()
    }
}