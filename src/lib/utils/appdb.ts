import Database from '@tauri-apps/plugin-sql';


export class AppDB {
    #db!: Database;

    async init() {
        this.#db = await Database.load("sqlite:app.db");
        await this.#db.execute('select 1');
    }

}

export const appDB = new AppDB();