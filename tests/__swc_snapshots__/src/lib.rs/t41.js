export default {
    '/test': async ()=><Example user={await _$(async ()=>await getCurrentUser())}/>
};
