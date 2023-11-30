use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Routes>
                <Route path="" view=move || view! { <Home/> }/>
            </Routes>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <Title text="Robin Thöne"/>
        <Html class="dark"/>
        <main>
            <div class="flex justify-center min-h-screen bg-white text-black dark:bg-black dark:text-white">
                <div class="max-w-screen-2xl">
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Consequat nisl vel pretium lectus. Malesuada fames ac turpis egestas sed tempus. Sagittis id consectetur purus ut faucibus pulvinar. Erat nam at lectus urna duis. Dictumst quisque sagittis purus sit amet volutpat. Vitae elementum curabitur vitae nunc sed. In mollis nunc sed id semper risus in. Vel pharetra vel turpis nunc eget lorem dolor sed. Adipiscing enim eu turpis egestas pretium aenean. Pellentesque adipiscing commodo elit at imperdiet dui. Euismod lacinia at quis risus sed. Sed id semper risus in hendrerit gravida. Gravida in fermentum et sollicitudin ac. Sed ullamcorper morbi tincidunt ornare. Urna porttitor rhoncus dolor purus non enim praesent elementum. Sit amet tellus cras adipiscing enim eu. Elit ullamcorper dignissim cras tincidunt lobortis feugiat vivamus. A cras semper auctor neque vitae. Egestas congue quisque egestas diam in.

                    Lacus luctus accumsan tortor posuere ac ut consequat semper. Sed turpis tincidunt id aliquet risus feugiat in. Pellentesque habitant morbi tristique senectus et netus. Lobortis mattis aliquam faucibus purus in massa. Volutpat sed cras ornare arcu dui vivamus. Scelerisque felis imperdiet proin fermentum leo vel. Malesuada bibendum arcu vitae elementum curabitur vitae nunc sed. Neque ornare aenean euismod elementum nisi quis eleifend quam. Scelerisque varius morbi enim nunc faucibus a pellentesque sit. Rhoncus est pellentesque elit ullamcorper dignissim cras tincidunt. Nisl tincidunt eget nullam non nisi est sit amet facilisis. Purus gravida quis blandit turpis cursus in hac habitasse. Quam quisque id diam vel quam elementum pulvinar etiam non. Eget nunc scelerisque viverra mauris. Morbi leo urna molestie at elementum eu facilisis sed odio. Convallis posuere morbi leo urna molestie at. Elementum facilisis leo vel fringilla est.

                    Pellentesque habitant morbi tristique senectus et netus et malesuada fames. Consectetur a erat nam at lectus urna duis. Ultrices sagittis orci a scelerisque. Felis donec et odio pellentesque diam volutpat. Mattis molestie a iaculis at erat. Aliquet risus feugiat in ante. Amet risus nullam eget felis. Sed vulputate mi sit amet mauris commodo quis. Elementum tempus egestas sed sed risus pretium. Tellus at urna condimentum mattis pellentesque. Nisl pretium fusce id velit ut tortor. Et magnis dis parturient montes nascetur ridiculus. At lectus urna duis convallis convallis tellus id interdum. Quam nulla porttitor massa id neque aliquam.

                    Diam vulputate ut pharetra sit amet. Volutpat est velit egestas dui id ornare arcu. Nulla porttitor massa id neque aliquam. Pellentesque elit ullamcorper dignissim cras tincidunt. Amet tellus cras adipiscing enim eu turpis egestas pretium. Odio tempor orci dapibus ultrices in iaculis nunc. Turpis massa tincidunt dui ut ornare. Libero nunc consequat interdum varius sit. Facilisi etiam dignissim diam quis enim. Fringilla phasellus faucibus scelerisque eleifend donec. Turpis tincidunt id aliquet risus feugiat. Dolor purus non enim praesent elementum. Et ultrices neque ornare aenean euismod elementum nisi quis. In hac habitasse platea dictumst quisque sagittis. Arcu non sodales neque sodales ut etiam sit. Volutpat ac tincidunt vitae semper quis. In nibh mauris cursus mattis. Facilisi etiam dignissim diam quis enim lobortis. Pulvinar neque laoreet suspendisse interdum consectetur libero id faucibus.

                    Elit sed vulputate mi sit amet mauris commodo. Aliquet eget sit amet tellus. Sapien nec sagittis aliquam malesuada bibendum arcu vitae. Eget lorem dolor sed viverra ipsum nunc aliquet. Tempus urna et pharetra pharetra massa massa ultricies. Sit amet consectetur adipiscing elit pellentesque habitant morbi tristique. Sed risus pretium quam vulputate dignissim suspendisse in. Semper risus in hendrerit gravida rutrum quisque non tellus orci. Tempus quam pellentesque nec nam. Donec ac odio tempor orci. Ut ornare lectus sit amet est. Justo donec enim diam vulputate ut pharetra sit. Molestie ac feugiat sed lectus vestibulum mattis. Risus ultricies tristique nulla aliquet enim tortor at auctor urna.

                    Iaculis urna id volutpat lacus laoreet non curabitur. Risus in hendrerit gravida rutrum quisque. Fringilla urna porttitor rhoncus dolor purus non enim praesent. Sit amet cursus sit amet dictum sit amet justo donec. Mattis nunc sed blandit libero volutpat sed cras. Lobortis elementum nibh tellus molestie. Massa sapien faucibus et molestie. Augue mauris augue neque gravida in fermentum et sollicitudin. Risus pretium quam vulputate dignissim. Platea dictumst vestibulum rhoncus est pellentesque elit. Leo urna molestie at elementum eu facilisis. Eget felis eget nunc lobortis mattis aliquam faucibus. Sem viverra aliquet eget sit amet tellus cras.

                    Et magnis dis parturient montes nascetur ridiculus mus mauris. Magna ac placerat vestibulum lectus mauris ultrices. Dictumst quisque sagittis purus sit. Sed ullamcorper morbi tincidunt ornare massa eget egestas purus viverra. Ac turpis egestas sed tempus urna. At ultrices mi tempus imperdiet nulla. Aenean sed adipiscing diam donec adipiscing tristique risus. Tincidunt praesent semper feugiat nibh sed pulvinar proin gravida hendrerit. Sit amet massa vitae tortor condimentum lacinia quis vel eros. Amet consectetur adipiscing elit ut aliquam purus. Tempor id eu nisl nunc mi ipsum faucibus. Sed felis eget velit aliquet sagittis id. Elit eget gravida cum sociis natoque. Tortor dignissim convallis aenean et tortor at.

                    Porta nibh venenatis cras sed felis eget. Blandit massa enim nec dui nunc mattis enim ut tellus. Pulvinar neque laoreet suspendisse interdum consectetur libero. Amet porttitor eget dolor morbi non arcu. Nunc sed augue lacus viverra vitae congue eu. Nulla aliquet enim tortor at auctor urna nunc id. Tristique nulla aliquet enim tortor at. Quam nulla porttitor massa id neque aliquam vestibulum. Mauris vitae ultricies leo integer. Pharetra vel turpis nunc eget lorem. Platea dictumst quisque sagittis purus.

                    Ultricies mi quis hendrerit dolor magna eget est. In pellentesque massa placerat duis. Egestas purus viverra accumsan in nisl nisi scelerisque eu ultrices. Dictum varius duis at consectetur lorem donec massa sapien faucibus. Faucibus turpis in eu mi bibendum neque. Posuere sollicitudin aliquam ultrices sagittis orci a scelerisque purus. Tortor consequat id porta nibh venenatis cras. Donec massa sapien faucibus et molestie ac feugiat sed lectus. Dolor sed viverra ipsum nunc. Consectetur libero id faucibus nisl. Ullamcorper malesuada proin libero nunc consequat. Eros in cursus turpis massa. Pretium aenean pharetra magna ac placerat vestibulum lectus mauris. Aliquam eleifend mi in nulla posuere sollicitudin aliquam. Duis tristique sollicitudin nibh sit amet. Orci nulla pellentesque dignissim enim sit amet venenatis urna. Vulputate eu scelerisque felis imperdiet.

                    Duis convallis convallis tellus id interdum velit laoreet id. Pretium fusce id velit ut tortor pretium viverra suspendisse potenti. Enim nec dui nunc mattis enim ut tellus elementum. Mollis nunc sed id semper risus in hendrerit gravida rutrum. Id porta nibh venenatis cras. Tincidunt praesent semper feugiat nibh. Mauris augue neque gravida in. Blandit aliquam etiam erat velit scelerisque in dictum non. Non sodales neque sodales ut etiam sit. Auctor urna nunc id cursus metus. Ac tortor vitae purus faucibus ornare. Eget aliquet nibh praesent tristique magna. Dapibus ultrices in iaculis nunc sed augue lacus. Pellentesque eu tincidunt tortor aliquam. At varius vel pharetra vel turpis nunc eget. Ut tortor pretium viverra suspendisse potenti nullam ac. Nulla porttitor massa id neque. Vitae auctor eu augue ut lectus arcu bibendum. Mattis rhoncus urna neque viverra justo nec ultrices. Eget dolor morbi non arcu risus quis.

                </div>
            </div>
        </main>
    }
}

