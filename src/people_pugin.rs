use std::thread::sleep;
use std::time::Duration;
use bevy::prelude::*;
use bevy::ui::AlignSelf::Start;

pub struct PeoplePlugin;
impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, add_people)
            .add_systems(Update, ( print_names, people_with_jobs,people_ready_for_hire, person_does_job));
    }

}


#[derive(Component)]
pub struct Person {
    pub name: String,
}
#[derive(Component)]
pub struct Employed{
    pub job: Job,
}
#[derive(Debug)]
pub enum Job{
    Doctor,
    Firefighter,
    Lawyer,
    Jobless
}

pub fn add_people(mut commands:Commands){
    commands.spawn((
        Person{name:String::from("James")},
        Employed{job:Job::Firefighter},)



    );
    commands.spawn((
        Person{name:String::from("Alex")},
        Employed{job:Job::Lawyer}));
    commands.spawn((Person{name:String::from("David")},
                    Employed{job:Job::Doctor}));
    commands.spawn((Person{name:String::from("Peter")},
                    Employed{job:Job::Jobless}));
    commands.spawn((Person{name:String::from("Walentyn")},));

    sleep(Duration::from_secs(5));
}

pub fn print_names(person_guery: Query<&Person,>){
    for person in person_guery.iter(){
        println!("Name: {}", person.name);
    }
}

pub fn people_with_jobs(
    person_query: Query<&Person, With<Employed>>
){
    for person in person_query.iter(){
        println!("Name: {} has a job", person.name);
    }
}

pub fn people_ready_for_hire(person_query: Query<&Person, Without<Employed>>){
    for person in person_query.iter(){
        println!("Name: {} ready for hire", person.name);
    }
}

pub fn person_does_job(
    person_query: Query<(&Person,&Employed)>
){
    for (person,employed) in person_query.iter(){
        let job_name = match employed.job{
            Job::Doctor => "Doctor",
            Job::Firefighter => "Fire Fighter",
            Job::Lawyer => "Lawyer",
            Job::Jobless => "Jobless",
        };
        println!("Name: {} is emplyed as {}", person.name, job_name);

    }
}