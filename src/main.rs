use chrono::Timelike;
use leptos::*;
use leptos_router::*;

fn main() {
    mount_to_body(|| {
        view! {
            <div class="flex-grow">
                <div class="container mx-auto px-4 py-8 max-w-3xl">
                    <div class="text-xl mb-4">"Time Bank Progress"</div>
                    <AppRouter />
                </div>
            </div>

            <footer class="container mx-auto px-4 py-8">
                <div>
                    "Made with ❤️ by "
                    <a class="underline" href="https://philippeloctaux.com">"phil"</a>" with "
                    <a class="underline" href="https://leptos.dev">"leptos"</a>
                    ", see the "
                    <a class="underline" href="https://github.com/x4m3/time-bank-progress">"source code"</a>
                </div>
            </footer>
        }
    })
}

fn github_pages_route<S: Into<String>>(url: S) -> String {
    if cfg!(debug_assertions) {
        url.into()
    } else {
        format!("/time-bank-progress{}", url.into())
    }
}

#[component]
fn AppRouter() -> impl IntoView {
    view! {
        <Router>
            <nav class="flex sm:justify-center items-center space-x-4">
                <a class="rounded-lg px-3 py-2 font-medium hover:bg-slate-100 hover:text-slate-900" href=github_pages_route("/")>"Home"</a>
                <a class="rounded-lg px-3 py-2 font-medium hover:bg-slate-100 hover:text-slate-900" href=github_pages_route("/quote")>"Quote"</a>
                <a class="rounded-lg px-3 py-2 font-medium hover:bg-slate-100 hover:text-slate-900" href=github_pages_route("/timezone")>"Update timezone"</a>
            </nav>

            <main class="mt-8">
                <Routes>
                    <Route path=github_pages_route("/") view=App />
                    <Route path=github_pages_route("/quote") view=Quote />
                    <Route path=github_pages_route("/timezone") view=Timezone />
                    <Route path=github_pages_route("/*any") view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Quote() -> impl IntoView {
    view! {
        <div class="text-3xl">"Quote"</div>

        <div class="prose dark:prose-invert mx-auto">
            <blockquote>
                <p>"Imagine there is a bank account that credits your account each morning with $86,400. It carries over no balance from day to day."</p>
                <p>"Every evening the bank deletes whatever part of the balance you failed to used during the day. What would you do? Draw out every cent, of course? Each of us has such a bank, it’s name is time."</p>
                <p>"Every morning, it credits you 86,400 seconds. Every night it writes off at a lost, whatever of this you failed to invest to a good purpose. It carries over no balance."</p>
                <p>"It allows no over draft. Each day it opens a new account for you. Each night it burns the remains of the day."</p>
                <p>"If you fail to use the day’s deposits, the loss is yours. There is no drawing against “tomorrow”. You must live in the present on today’s deposits."</p>
                <p>"Invest it so as to get from it the utmost in health, happiness, and health. The clock is running. Make the most of today."</p>
                <h2>"Marc Levy"</h2>
            </blockquote>
        </div>
    }
}

#[component]
fn Timezone() -> impl IntoView {
    // Put UTC at the top
    let timezones_utc = ["UTC"];

    // List other possible timezones
    let timezones = chrono_tz::TZ_VARIANTS
        .into_iter()
        .filter(|tz| tz.name().contains('/'))
        .filter(|tz| !tz.name().starts_with("Etc"))
        .map(|timezone| timezone.name());

    let all_options =
        // Combine all options together
        timezones_utc
            .into_iter()
            .chain(timezones)
            // Create their view
            .map(|tz| {
                // Link with timezone as query
                let encoded_timezone = url::form_urlencoded::byte_serialize(tz.as_bytes()).collect::<String>();
                let uri = github_pages_route(format!("/?tz={}", encoded_timezone));
                view! { <a class="block my-1" href=uri>{tz}</a> }
            })
            .collect_view();

    view! {
        <div class="text-3xl">"Choose your timezone"</div>
        <div>{all_options}</div>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    view! {
        <div class="text-3xl">"404 Not found"</div>
    }
}

#[component]
fn App() -> impl IntoView {
    // Get query "tz" into a String
    let query = use_query_map();
    let tz_query = move || query.with(|query| query.get("tz").cloned());

    // Parse query to timezone, UTC if it does not exist or does not parse
    let timezone = move || {
        tz_query()
            .unwrap_or("UTC".into())
            .parse::<chrono_tz::Tz>()
            .unwrap_or(chrono_tz::UTC)
    };

    let now = chrono::Utc::now;
    let (timestamp, set_timestamp) = create_signal(now());
    let timestamp_tz = move || timestamp.get().with_timezone(&timezone());

    // Refresh time often
    set_interval(
        move || set_timestamp.set(now()),
        std::time::Duration::from_millis(90),
    );

    // Pretty date and time
    let date = move || timestamp_tz().format("%A %d %B").to_string();
    let time = move || timestamp_tz().format("%T%.3f %Z").to_string();

    // Calculate time left in day
    let end_of_day = move || {
        chrono::Duration::days(1).num_seconds() as u32 - timestamp_tz().num_seconds_from_midnight()
    };

    view! {
        // Current date and time
        <div class="space-y-2">
            <div class="text-4xl">{date}</div>
            <div class="text-3xl font-mono">{time}</div>
        </div>

        // Balance in bank account
        <div class="mt-8 font-mono">
            <p class="text-5xl">"$"{end_of_day}</p>
            <p class="mt-2">"available in your bank account"</p>
        </div>
    }
}
