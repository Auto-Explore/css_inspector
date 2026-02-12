# css/css-mixins/dashed-function-eval.html

```json
{
  "format_version": 3,
  "file": "css/css-mixins/dashed-function-eval.html"
}
```

## style[0]

```css

    @function --f() {
      result: 12px;
    }
    #target {
      --actual: --f();
      --expected: 12px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[1]

```css

    @function --f() returns <length> {
      result: 12px;
    }
    #target {
      --actual: --f();
      --expected: 12px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[2]

```css

    @function --f() returns <length> {
      result: calc(12px + 1px);
    }
    #target {
      --actual: --f();
      --expected: 13px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[3]

```css

    @function --f() returns <length> {
      result: 12s;
    }
    #target {
      --actual: --f();
      /* --expected: <guaranteed-invalid> */
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[4]

```css

    @function --f() {
    }
    #target {
      --actual: --f();
      /* --expected: <guaranteed-invalid> */
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[5]

```css

    @function --f() {
      result:;
    }
    #target {
      --actual: --f();
      --expected:;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[6]

```css

    @function --f() returns <length> {
      result: 12px;
      result: 24px; /* Overwrites previous */
    }
    #target {
      --actual: --f();
      --expected: 24px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[7]

```css

    @function --f() {
      result: --g();
    }
    @function --g() {
      result: 12px;
    }
    #target {
      --actual: --f();
      --expected: 12px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[8]

```css

    @function --f(--x) {
      result: 12px;
    }
    #target {
      --actual: --f(100px);
      --expected: 12px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[9]

```css

    @function --f(--x) {
      result: var(--x);
    }
    #target {
      --actual: --f(100px);
      --expected: 100px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[10]

```css

    @function --f(--x, --y, --z) {
      result: var(--x) var(--y) var(--z);
    }
    #target {
      --actual: --f(100px, auto, red);
      --expected: 100px auto red;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[11]

```css

    @function --f(--x <length>) {
      result: var(--x);
    }
    #target {
      --actual: --f(100px);
      --expected: 100px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[12]

```css

    @function --f(--x <length>) {
      result: var(--x);
    }
    #target {
      --actual: --f(calc(100px + 1px));
      --expected: 101px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[13]

```css

    @function --f(--x type(*)) {
      result: var(--x);
    }
    #target {
      --actual: --f(calc(100px + 1px));
      --expected: calc(100px + 1px);
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[14]

```css

    @function --f(--x <length>, --y <angle>, --z <time>) {
      result: var(--x) var(--y) var(--z);
    }
    #target {
      --actual: --f(calc(100px + 1px), 1turn, 1000ms);
      --expected: 101px 360deg 1s;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[15]

```css

    @function --f(--x type(<length> | auto)) {
      result: var(--x);
    }
    #target {
      --actual: --f(auto);
      --expected: auto;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[16]

```css

    @function --f(--x type(<length> | auto)) {
      result: var(--x);
    }
    #target {
      --actual: --f(10px);
      --expected: 10px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[17]

```css

    @function --f(--x) {
      result: --g(var(--x));
    }
    @function --g(--y) {
      result: var(--y);
    }
    #target {
      --actual: --f(12px);
      --expected: 12px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[18]

```css

    @function --f(--x) {
      --one: FAIL;
      result: var(--x);
    }
    #target {
      --one: 1px;
      --actual: --f(calc(100px + var(--one)));
      --expected: calc(100px + 1px);
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[19]

```css

    @function --f(--x <length>) {
      --one: FAIL;
      result: var(--x);
    }
    #target {
      --one: 1px;
      --actual: --f(calc(100px + var(--one)));
      --expected: 101px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[20]

```css

    @function --f(--x) {
      result: var(--x, PASS);
    }
    #target {
      --actual: --f(var(--unknown));
      --expected: PASS;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[21]

```css

    @function --f(--x <length>) {
      result: var(--x, PASS);
    }
    #target {
      --actual: --f(var(--unknown));
      --expected: PASS;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[22]

```css

    @function --f(--x <length>) {
      result: var(--x, PASS);
    }
    #target {
      --actual: --f(red);
      --expected: PASS;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[23]

```css

    @function --f(--x: PASS) {
      result: var(--x);
    }
    #target {
      --actual: --f();
      --expected: PASS;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[24]

```css

    @function --f(--x, --y: 2px, --z: 3px) {
      result: var(--x) var(--y) var(--z);
    }
    #target {
      --actual: --f(1px, 5px);
      --expected: 1px 5px 3px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[25]

```css

    @function --f(--x, --y <length>: 2px, --z <length>: 3px) {
      result: var(--x) var(--y) var(--z);
    }
    #target {
      --actual: --f(1px, 5px);
      --expected: 1px 5px 3px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[26]

```css

    @function --f(--x, --y: var(--x)) {
      result: var(--x) var(--y);
    }
    #target {
      --x: FAIL;
      --y: FAIL;
      --actual: --f(5px);
      --expected: 5px 5px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[27]

```css

    @function --f(--x, --y: var(--x)) {
      --x: 17px;
      result: var(--x) var(--y);
    }
    #target {
      --x: FAIL;
      --y: FAIL;
      --actual: --f(5px);
      --expected: 17px 5px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[28]

```css

    @function --f(--x: 5px, --y: var(--x)) {
      result: var(--x) var(--y);
    }
    #target {
      --x: FAIL;
      --y: FAIL;
      --actual: --f();
      --expected: 5px 5px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[29]

```css

    @function --f(--x: 5px, --y <length>: calc(var(--x) + 1px)) {
      result: var(--x) var(--y);
    }
    #target {
      --x: FAIL;
      --y: FAIL;
      --actual: --f();
      --expected: 5px 6px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[30]

```css

    @function --f(--x: 1, --y, --z: 3) {
      result: var(--x) var(--y) var(--z);
    }
    #target {
      --actual: --f(var(--unknown), 2, var(--unknown));
      --expected: 1 2 3;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[31]

```css

    @function --f(--x <number>: 1, --y <number>, --z <number>: 3) {
      result: var(--x) var(--y) var(--z);
    }
    #target {
      --actual: --f(var(--unknown), 2, var(--unknown));
      --expected: 1 2 3;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[32]

```css

    @function --f(--x <number>: 1, --y <number>, --z <number>: 3) {
      result: var(--x) var(--y) var(--z);
    }
    #target {
      --actual: --f(red, 2, 360deg);
      --expected: 1 2 3;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[33]

```css

    @function --f() {
      --x: 10px;
      result: 1px;
    }
    #target {
      --actual: --f();
      --expected: 1px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[34]

```css

    @function --f() {
      --x: 10px;
      result: 1px;
    }
    #target {
      --x: 20px;
      --actual: --f() var(--x);
      --expected: 1px 20px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[35]

```css

    @function --f() {
      --x: 10px;
      result: var(--x);
    }
    #target {
      --actual: --f();
      --expected: 10px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[36]

```css

    @function --f() {
      --x: 10px;
      --y: 17px;
      result: var(--x) var(--y);
    }
    #target {
      --actual: --f();
      --expected: 10px 17px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[37]

```css

    @function --f() {
      --x: 10px;
      --y: var(--x);
      result: var(--y);
    }
    #target {
      --actual: --f();
      --expected: 10px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[38]

```css

    @function --f() {
      result: var(--y);
      --x: 10px;
      --y: var(--x);
    }
    #target {
      --actual: --f();
      --expected: 10px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[39]

```css

    @function --f() {
      --x: 10px;
      --y: var(--x);
      result: var(--y);

      --x: 20px; /* Surprise! */
    }
    #target {
      --actual: --f();
      --expected: 20px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[40]

```css

    @function --f() {
      result: var(--x);
    }
    #target {
      --x: 10px;
      --actual: --f();
      --expected: 10px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[41]

```css

    @function --f() {
      --x: PASS;
      result: --g();
    }
    @function --g() {
      result: var(--x);
    }
    #target {
      --x: FAIL;
      --actual: --f();
      --expected: PASS;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[42]

```css

    @function --f(--x) {
      result: --g();
    }
    @function --g() {
      result: var(--x);
    }
    #target {
      --x: FAIL;
      --actual: --f(PASS);
      --expected: PASS;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[43]

```css

    @function --f(--x) {
      result: --g(PASS);
    }
    @function --g(--x) {
      result: var(--x);
    }
    #target {
      --x: FAIL;
      --actual: --f(FAIL);
      --expected: PASS;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[44]

```css

    @function --f() {
      --x: FAIL;
      result: --g(PASS);
    }
    @function --g(--x) {
      result: var(--x);
    }
    #target {
      --x: FAIL;
      --actual: --f();
      --expected: PASS;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[45]

```css

    @function --f(--x) {
      result: --g();
    }
    @function --g() {
      --x: PASS;
      result: var(--x);
    }
    #target {
      --x: FAIL;
      --actual: --f(FAIL);
      --expected: PASS;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[46]

```css

    @function --f() {
      --x: FAIL;
      result: --g();
    }
    @function --g() {
      --x: PASS;
      result: var(--x);
    }
    #target {
      --x: FAIL;
      --actual: --f();
      --expected: PASS;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[47]

```css

    @function --f() {
      --y: 1;
      --x: var(--y);
      result: --g();
    }
    @function --g() {
      --y: 0;
      result: var(--x);
    }
    #target {
      --y: 0;
      --x: FAIL;
      --actual: --f();
      --expected: 1;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[48]

```css

    @function --f(--l <length>: 10.00px) {
      result: --g();
    }
    @function --g() {
      result: var(--l);
    }
    #target {
      --actual: --f();
      --expected: 10px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[49]

```css

    @function --one() {
      --x: 1;
      result: --f();
    }
    @function --two() {
      --x: 2;
      result: --f();
    }
    @function --three() {
      --x: 3;
      result: --f();
    }
    @function --f() {
      result: var(--x);
    }
    #target {
      --x: 0;
      --actual: --one() --two() --three() --f();
      --expected: 1 2 3 0;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[50]

```css

    @function --a() {
      --x: 1;
      result: --b();
    }
    @function --b() {
      result: --c();
    }
    @function --c() {
      result: var(--x);
    }
    #target {
      --x: 0;
      --actual: --a();
      --expected: 1;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[51]

```css

    @function --a() {
      --x: var(--unknown);
      result: --b();
    }
    @function --b() {
      result: var(--x, PASS);
    }
    #target {
      --x: FAIL;
      --actual: --a();
      --expected: PASS;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[52]

```css

    @function --a() {
      --x: --b();
      --y: var(--px);
      result: var(--x);
    }

    @function --b() {
      result: var(--y, FAIL);
    }
    #target {
      --px: 10px;
      --actual: --a();
      --expected: 10px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[53]

```css

    @function --a() {
      --x: var(--px);
      --y: --b();
      result: var(--y);
    }

    @function --b() {
      result: var(--x, FAIL);
    }
    #target {
      --px: 10px;
      --actual: --a();
      --expected: 10px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[54]

```css

    @function --f(--x) {
      result: var(--x);
    }
    #target {
      --x: FAIL;
      --actual: --f(PASS);
      --expected: PASS;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[55]

```css

    @function --f(--x) {
      --x: PASS;
      result: var(--x);
    }
    #target {
      --x: FAIL1;
      --actual: --f(FAIL2);
      --expected: PASS;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[56]

```css

    @function --f(--x) {
      result: var(--x, PASS);
    }
    #target {
      --x: FAIL;
      --actual: --f(var(--unknown));
      --expected: PASS;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[57]

```css

    @function --f(--x <length>) {
      result: var(--x, PASS);
    }
    #target {
      --x: FAIL;
      --actual: --f(var(--unknown));
      --expected: PASS;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[58]

```css

    @function --f(--x <length>) {
      result: var(--x, PASS);
    }
    #target {
      --x: FAIL;
      --actual: --f(red);
      --expected: PASS;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[59]

```css

    @function --f(--x) {
      result: 10px;
    }
    #target {
      --actual: --f();
      /* --expected: <guaranteed-invalid> */
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[60]

```css

    @function --f(--x, --y, --z) {
      result: 10px;
    }
    #target {
      --actual: --f(1, 2);
      /* --expected: <guaranteed-invalid> */
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[61]

```css

    @function --f(--x) {
      result: var(--x);
    }
    #target {
      --actual: --f({1px,2px});
      --expected: 1px,2px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[62]

```css

    @function --f(--x, --y) {
      result: var(--x) | var(--y);
    }
    #target {
      --actual: --f({1px, 2px}, 3px);
      --expected: 1px, 2px | 3px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[63]

```css

    @function --f(--x, --y) {
      result: var(--x) | var(--y);
    }
    #target {
      --actual: --f(1px, {2px, 3px});
      --expected: 1px | 2px, 3px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[64]

```css

    @function --f(--x) {
      result: var(--x);
    }
    #target {
      --actual: --f({,});
      --expected: ,;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[65]

```css

    @function --f(--x) {
      result: var(--x);
    }
    #target {
      --actual: --f({{}});
      --expected: {};
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[66]

```css

    @function --f(--x) {
      result: var(--x);
    }
    #target {
      --actual: --f({foo{}});
      --expected: foo{};
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[67]

```css

    @function --f(--x: FAIL1) {
      --x: FAIL2;
      --x: initial;
      result: var(--x);
    }
    #target {
      --actual: --f(PASS);
      --expected: PASS;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[68]

```css

    @function --f(--x: PASS) {
      --x: FAIL;
      --x: initial;
      result: var(--x);
    }
    #target {
      --actual: --f();
      --expected: PASS;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[69]

```css

    @function --f(--x) {
      --x: FAIL;
      --x: initial;
      result: var(--x, PASS);
    }
    #target {
      --actual: --f(var(--unknown));
      --expected: PASS;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[70]

```css

    @function --f(--x: initial) {
      result: var(--x, PASS);
    }
    #target {
      --actual: --f();
      --expected: PASS;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[71]

```css

    @function --f(--x: PASS) {
      --x: var(--unknown, initial);
      result: var(--x);
    }
    #target {
      --actual: --f();
      --expected: PASS;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[72]

```css

    @function --f(--x: FAIL1) {
      --x: FAIL2;
      --x: inherit;
      result: var(--x);
    }
    #target {
      --x: PASS;
      --actual: --f(FAIL3);
      --expected: PASS;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[73]

```css

    @function --f(--x: FAIL1) {
      --x: FAIL2;
      --x: inherit;
      result: var(--x);
    }
    @function --g(--x) {
      --x: PASS;
      result: --f(FAIL3);
    }
    #target {
      --actual: --g(FAIL4);
      --expected: PASS;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[74]

```css

    @function --f(--x) {
      --x: FAIL2;
      --x: inherit;
      result: var(--x, PASS);
    }
    @function --g() {
      --x: var(--unknown);
      result: --f(FAIL1);
    }
    #target {
      --actual: --g();
      --expected: PASS;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[75]

```css

    @function --f(--x: inherit) {
      result: var(--x);
    }
    #target {
      --x: PASS1;
      --actual: --f() --f(PASS2);
      --expected: PASS1 PASS2;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[76]

```css

    @function --f(--x: inherit) {
      result: var(--x);
    }
    @function --g() {
      --x: PASS1;
      result: --f() --f(PASS2);
    }
    #target {
      --x: FAIL;
      --actual: --g();
      --expected: PASS1 PASS2;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[77]

```css

    @function --f() {
      --x: unset;
      result: var(--x, PASS);
    }
    #target {
      --actual: --f();
      --expected: PASS;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[78]

```css

    @function --f() {
      --x: revert;
      result: var(--x, PASS);
    }
    #target {
      --actual: --f();
      --expected: PASS;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[79]

```css

    @function --f() {
      --x: revert-layer;
      result: var(--x, PASS);
    }
    #target {
      --actual: --f();
      --expected: PASS;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[80]

```css

    @function --f() {
      --x: revert-rule;
      result: var(--x, PASS);
    }
    #target {
      --actual: --f();
      --expected: PASS;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[81]

```css

    @function --f() {
      result: initial;
    }
    #target {
      --tmp: --f();
      --actual: var(--tmp, PASS);
      --expected: PASS;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[82]

```css

    @function --f() {
      result: inherit;
    }
    #parent {
      --tmp: PASS;
    }
    #target {
      --tmp: --f();
      --actual: var(--tmp, FAIL);
      --expected: PASS;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[83]

```css

    @function --f() {
      result: unset;
    }
    #parent {
      --tmp: PASS;
    }
    #target {
      --tmp: --f();
      --actual: var(--tmp, FAIL);
      --expected: PASS;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[84]

```css

    @function --f() {
      result: revert;
    }
    #parent {
      --tmp: PASS;
    }
    #target {
      --tmp: --f();
      --actual: var(--tmp, FAIL);
      --expected: PASS;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[85]

```css

    @function --f() {
      result: revert-layer;
    }
    @layer one {
      #target {
        --tmp: PASS;
      }
    }
    @layer two {
      #target {
        --tmp: --f();
        --actual: var(--tmp, FAIL);
        --expected: PASS;
      }
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[86]

```css

    @function --f() {
      result: revert-rule;
    }
    #target {
      --tmp: PASS;
    }
    #target {
      --tmp: --f();
      --actual: var(--tmp, FAIL);
      --expected: PASS;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[87]

```css

    @function --f() {
      result: initial;
    }
    @function --g(--x: PASS) {
      --x: FAIL1;
      --x: --f();
      result: var(--x, FAIL2);
    }
    #target {
      --actual: --g();
      --expected: PASS;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[88]

```css

    @function --f() returns <length> {
      result: revert-layer;
    }
    @layer one {
      #target {
        --tmp: FAIL;
      }
    }
    @layer two {
      #target {
        --tmp: --f();
        --actual: var(--tmp, PASS);
        --expected: PASS;
      }
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
