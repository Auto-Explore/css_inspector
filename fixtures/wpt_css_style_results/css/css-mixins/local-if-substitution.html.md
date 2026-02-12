# css/css-mixins/local-if-substitution.html

```json
{
  "format_version": 3,
  "file": "css/css-mixins/local-if-substitution.html"
}
```

## style[0]

```css

    @function --f() {
      --x: 3px;
      result: if(style(--x: 3px): PASS; else: FAIL;);
    }
    #target {
      --x: 1px;
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
      --x: 3px;
      result: if(style(--y: var(--x)): PASS; else: FAIL;);
    }
    #target {
      --x: 1px;
      --y: 3px;
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

## style[2]

```css

    @function --f() {
      --x: PASS;
      result: if(style(--true): var(--x); else: FAIL;);
    }
    #target {
      --true: true;
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

## style[3]

```css

    @function --f(--x) {
      result: if(style(--x: 3px): PASS; else: FAIL;);
    }
    #target {
      --x: 1px;
      --actual: --f(3px);
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

    @function --f(--x) {
      result: if(style(--y: var(--x)): PASS; else: FAIL;);
    }
    #target {
      --x: 1px;
      --y: 3px;
      --actual: --f(3px);
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

    @function --f(--x) {
      result: if(style(--true): var(--x); else: FAIL;);
    }
    #target {
      --true: true;
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

## style[6]

```css

    @function --f() {
      result: if(style(--true): --g(); else: FAIL;);
    }
    @function --g() {
      result: PASS;
    }
    #target {
      --true: true;
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

## style[7]

```css

    @function --f(--x) {
      --true: true;
      result: if(style(--true): --g(var(--x)); else: FAIL;);
    }
    @function --g(--x) {
      result: var(--x, FAIL);
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

## style[8]

```css

    @function --f() {
      --x: if(style(--true): var(--x); else: FAIL;);
      result: var(--x, PASS);
    }
    #target {
      --true: true;
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

## style[9]

```css

    @function --f() {
      --x: if(style(--x): FAIL1; else: FAIL2;);
      result: var(--x, PASS);
    }
    #target {
      --x: 1px;
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
      --x: if(style(--y: var(--x)): FAIL1; else: FAIL2;);
      result: var(--x, PASS);
    }
    #target {
      --y: 1px;
      --x: 1px;
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
      --local: --g();
      result: var(--local);
    }
    @function --g() {
      result: if(style(--true): --f());
    }
    #target {
      --true: true;
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

## style[12]

```css

    @function --f() {
      --x: 3px;
      result: if(style(--x): PASS; else: FAIL);
    }
    #target {
      --x: var(--x);
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

## style[13]

```css

    @function --f(--x) {
      result: if(style(--x): PASS; else: FAIL);
    }
    #target {
      --x: var(--x);
      --actual: --f(3px);
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

    @function --f(--c: green) {
      result: if(style(--c: initial): PASS; else: FAIL);
    }
    @function --g() {
      --c: red;
      result: --f();
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

## style[15]

```css

    @function --f(--c: red) {
      --c: green;
      result: if(style(--c: inherit): PASS; else: FAIL);
    }
    @function --g() {
      --c: green;
      result: --f();
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

## style[16]

```css

    @function --f() {
      result: if(style(--c: initial): PASS; else: FAIL);
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

## style[17]

```css

    @function --f() {
      result: if(style(--c: unset): PASS; else: FAIL);
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

## style[18]

```css

    @function --f() {
      result: if(style(--c: revert): FAIL; else: PASS);
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

## style[19]

```css

    @function --f() {
      result: if(style(--c: revert-layer): FAIL; else: PASS);
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
