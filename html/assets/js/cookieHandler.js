/**
 * This JS-File handles saving and loading from browser cookies
 */

Cookies.set('name', 'value');

console.log(Cookies.get('name')); // => 'value'
console.log(Cookies.get('nothing')); // => undefined

Cookies.remove('name');