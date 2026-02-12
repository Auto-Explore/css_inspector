# css/css-mixins/function-parameter-types.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-mixins/function-parameter-types.tentative.html"
}
```

## style[0]

```css

    @function --f(--c <color>) {
      result: if(
        style(--c:red): PASS;
        else: FAIL);
    }
    #target {
      --actual: --f(#f00);
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

    @function --f(--c <color>) {
      --c:#00f; /* <color> */
      result: if(
        style(--c:blue): PASS;
        else: FAIL);
    }
    #target {
      --actual: --f(#000);
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
      result: if(
        style(--c:red): PASS;
        else: FAIL);
    }
    @function --g(--c <color>) {
      result: --f();
    }
    #target {
      --actual: --g(#f00);
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

    @function --f() {
      result: if(
        style(--c:blue): PASS;
        else: FAIL);
    }
    @function --g(--c <color>) {
      --c:#00f; /* <color> */
      result: --f();
    }
    #target {
      --actual: --g(#f00);
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

    @function --f(--c type(*)) {
      result: if(
        style(--c:red): FAIL1;
        style(--c:#f00): FAIL2;
        style(--c:lime): FAIL3;
        style(--c:#0f0): PASS;
        else: FAIL4);
    }
    @function --g(--c <color>) {
      result: --f(#0f0);
    }
    #target {
      --actual: --g(#f00);
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

    @function --f(--c <color>) {
      --c: 3;
      result: var(--c, PASS);
    }
    #target {
      --actual: --f(#f00);
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

    @property --c {
      syntax: "<color>";
      inherits: false;
      initial-value: black;
    }
    @function --f() {
      result: if(style(--c:#f00): PASS; else: FAIL);
    }
    #target {
      --c: red;
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
