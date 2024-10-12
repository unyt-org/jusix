class X {
    method() {
        return <button onclick:frontend={()=>{
            use("silent-errors", this, data);
            return this.handleSettings(data);
        }}>
                Apply settings
            </button>;
    }
}
