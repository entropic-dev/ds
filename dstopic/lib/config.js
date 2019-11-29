import figgyPudding from 'figgy-pudding';
const config = figgyPudding({});
function getConfigs(argv) {
    return config(argv);
}
export default getConfigs;
