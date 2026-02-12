# css/css-mixins/local-inherit-substitution.html

```json
{
  "format_version": 3,
  "file": "css/css-mixins/local-inherit-substitution.html"
}
```

## style[0]

```css

    @function --f() {
      --x: FAIL2;
      result: inherit(--x);
    }
    #parent {
      --x: FAIL1;
    }
    #parent > #target {
      --x: PASS;
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
      --x: PASS;
      result: --g();
    }
    @function --g() {
      --x: FAIL3;
      result: inherit(--x);
    }
    #parent {
      --x: FAIL1;
    }
    #parent > #target {
      --x: FAIL2;
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
      --x: var(--noexist);
      result: --g();
    }
    @function --g() {
      --x: FAIL3;
      result: inherit(--x, PASS);
    }
    #parent {
      --x: FAIL1;
    }
    #parent > #target {
      --x: FAIL2;
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

    @function --f() {
      --x: var(--x);
      result: --g();
    }
    @function --g() {
      --x: FAIL3;
      result: inherit(--x, PASS);
    }
    #parent {
      --x: FAIL1;
    }
    #parent > #target {
      --x: FAIL2;
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

    @function --f(--x <length>) {
      result: --g();
    }
    @function --g() {
      --x: 14px;
      result: inherit(--x);
    }
    #parent {
      --x: 12px;
    }
    #parent > #target {
      --x: 13px;
      --actual: --f(42.0px);
      --expected: 42px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
