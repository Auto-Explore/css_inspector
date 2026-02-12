# css/css-mixins/dashed-function-cycles.html

```json
{
  "format_version": 3,
  "file": "css/css-mixins/dashed-function-cycles.html"
}
```

## style[0]

```css

    @function --f() {
      --x: var(--x);
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

## style[1]

```css

    @function --f() {
      --x: var(--x);
      result: var(--x);
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

## style[2]

```css

    @function --f() {
      --y: PASS;
      --x: var(--y, var(--x));
      result: var(--x, FAIL);
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

## style[3]

```css

    /* The _properties_ --x and --y are in a cycle.
      However, the _local_ --x is not a cycle with anything. */
    @function --f() {
      --x: var(--y, PASS);
      result: var(--x);
    }
    #target {
      --x: var(--y);
      --y: var(--x);
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

## style[4]

```css

    /* The locals --x and --y are in a cycle within --f(). */
    @function --f() {
      --x: var(--y);
      --y: var(--x);
      result: --g();
    }
    @function --g() {
      --x: var(--y, PASS); /* Shadows outer --x. */
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

## style[5]

```css

    @function --f() {
      --x: var(--x); /* Cycle */
      result: --g(10px);
    }
    @function --g(--x) {
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

## style[6]

```css

    @function --f(--x, --y) {
      result: var(--x) var(--y);
    }
    #target {
      --x: var(--y);
      --y: var(--x);
      --actual: --f(PASS-x, PASS-y);
      --expected: PASS-x PASS-y;
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
      result: var(--x, PASS-x) var(--x, PASS-y);
    }
    #target {
      --x: var(--y);
      --y: var(--x);
      --actual: --f();
      --expected: PASS-x PASS-y;
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

    @function --f() {
        --y: var(--x, 1);
        --x: var(--y, 3);
        result: var(--x) var(--y);
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

## style[9]

```css

    @function --f() {
      --x: var(--x);
      result: PASS;
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

## style[10]

```css

    @function --f() {
      --x: PASS;
      result: var(--x, var(--y));
    }
    #target {
      --y: var(--y);
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

## style[11]

```css

    @function --f() {
      --x: PASS;
      --y: var(--y);
      result: var(--x, var(--y));
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

## style[12]

```css

    @function --f() {
      result: --f();
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

## style[13]

```css

    @function --f() {
      result: --g();
    }
    @function --g() {
      result: --f();
    }
    #target {
      --tmp: --g();
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

## style[14]

```css

    @function --f() {
      result: --g();
    }
    @function --g() {
      result: --f();
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

## style[15]

```css

    @function --f() {
      --local: --f();
      result: var(--local);
    }
    #target {
      --local: FAIL;
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

## style[16]

```css

    @function --f() {
      --unused: --f();
      result: FAIL-result;
    }
    #target {
      --local: FAIL;
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

## style[17]

```css

    @function --f() {
      result: var(--global);
    }
    #target {
      --global: --f();
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

## style[18]

```css

    @function --f() {
      result: --g();
    }
    @function --g() {
      --local: --f();
      result: var(--local);
    }
    #target {
      --local: FAIL;
      --tmp: --g();
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

## style[19]

```css

    @function --f() {
      --a: --g();
      result: var(--a, PASS);
    }

    @function --g() {
      result: var(--a);
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

## style[20]

```css

    @function --f() {
      --local: --g();
      result: var(--local);
    }
    @function --g() {
      --local: FAIL;
      result: var(--global);
    }
    #target {
      --local: FAIL;
      --global: --f();
      --tmp: --g();
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

## style[21]

```css

    @function --f(--x, --y: --f(13px)) {
      result: 10px;
    }
    #target {
      --tmp: --f(42px);
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

## style[22]

```css

    @function --f(--x, --y: var(--z), --z: var(--y)) {
      result: var(--x, FAIL) var(--y, PASS-y) var(--z, PASS-z);
    }
    #target {
      --actual: --f(42px);
      --expected: 42px PASS-y PASS-z;
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

    @function --f() {
      --b: var(--b);
      --a: --g();
      result: var(--a);
    }

    @function --g(--a: var(--b)) {
      result: var(--a, PASS);
    }
    #target {
      --b: FAIL;
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

    @function --f() {
      --a: --g();
      result: var(--a);
    }

    @function --g() {
      --a: 10px;
      result: var(--a);
    }
    #target {
      /* Nothing is in a cycle here. */
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
