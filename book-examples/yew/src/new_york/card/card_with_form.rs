use yew::prelude::*;

use crate::new_york::components::ui::{
    button::{Button, ButtonVariant},
    card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle},
    label::Label,
};

#[function_component]
pub fn CardWithForm() -> Html {
    html! {
        <Card class="w-[350px]">
            <CardHeader>
                <CardTitle>{"Create project"}</CardTitle>
                <CardDescription>{"Deploy your new project in one-click."}</CardDescription>
            </CardHeader>
            <CardContent>
                <form>
                    <div class="grid w-full items-center gap-4">
                        <div class="flex flex-col space-y-1.5">
                            <Label r#for="name">{"Name"}</Label>
                            // TODO
                            // <Input id="name" placeholder="Name of your project" />
                        </div>
                        <div class="flex flex-col space-y-1.5">
                            <Label r#for="framework">{"Framework"}</Label>
                            // TODO
                            // <Select>
                            //     <SelectTrigger id="framework">
                            //         <SelectValue placeholder="Select" />
                            //     </SelectTrigger>
                            //     <SelectContent position="popper">
                            //         <SelectItem value="next">{"Next.js"}</SelectItem>
                            //         <SelectItem value="sveltekit">{"SvelteKit"}</SelectItem>
                            //         <SelectItem value="astro">{"Astro"}</SelectItem>
                            //         <SelectItem value="nuxt">{"Nuxt.js"}</SelectItem>
                            //     </SelectContent>
                            // </Select>
                        </div>
                    </div>
                </form>
            </CardContent>
            <CardFooter class="flex justify-between">
                <Button variant={ButtonVariant::Outline}>{"Cancel"}</Button>
                <Button>{"Deploy"}</Button>
            </CardFooter>
        </Card>
    }
}
