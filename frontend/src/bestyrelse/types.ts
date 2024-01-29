export type Person = {
    post: string,
    title: string,
    name: string,
    username: string,
    imageUrl: string,
};

export type Bestyrelse = {
    periode: string,
    members: Person[],
}