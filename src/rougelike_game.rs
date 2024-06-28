mod modern {
    trait Motion {
        fn pre(&self);
        fn play(&self);
        fn post(&self);
    }

    trait CanCutPre { 
        fn cut_pre(&self);
    }
    trait CanCutPost { 
        fn cut_post(&self);
    }

    trait CalcAndExecOnMotionEffectByCanCut { 

    }
    trait CalcAndExecOnMotionEffectByCanCutPre { 

    }
    struct MotionEffectByCanCutPre { 
        pre: Box<dyn CanCutPre>,
    }
    struct MotionEffectByCanCutPost { 
        post: Box<dyn CanCutPost>,
    }   

    struct Aminiation { 
        motion: Box<dyn Motion>,
        effect: Box<dyn CalcAndExecOnMotionEffectByCanCut>,

    }
}


pub fn run() {}
