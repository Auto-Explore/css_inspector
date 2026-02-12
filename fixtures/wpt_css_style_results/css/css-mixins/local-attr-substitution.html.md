# css/css-mixins/local-attr-substitution.html

```json
{
  "format_version": 3,
  "file": "css/css-mixins/local-attr-substitution.html"
}
```

## style[0]

```css

    @function --f() {
      --x: PASS;
      result: attr(data-x type(*));
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

## style[1]

```css

    @function --f(--x) {
      result: attr(data-x type(*));
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

## style[2]

```css

    @function --f() returns <length> {
      --x: calc(10px + 2px);
      result: attr(data-x type(<length>));
    }
    #target {
      --x: calc(10px + 1px);
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

## style[3]

```css

    @function --f() {
      --x: PASS;
      result: attr(data-unknown, var(--x));
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

## style[4]

```css

    @function --f() {
      --x: attr(data-x type(*));
      --y: attr(data-x type(*), PASS);
      result: var(--y, PASS);
    }
    #target {
      --x: FAIL1;
      --y: FAIL2;
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
      --valid: PASS;
      --x: var(--valid, attr(data-x type(*)));
      --y: attr(data-x type(*), FAIL);
      result: var(--y, FAIL);
    }
    #target {
      --x: FAIL1;
      --y: FAIL2;
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

## style[6]

```css

    @function --f() {
      --local: --g();
      result: var(--local);
    }
    @function --g() {
      result: attr(data-f type(*));
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
