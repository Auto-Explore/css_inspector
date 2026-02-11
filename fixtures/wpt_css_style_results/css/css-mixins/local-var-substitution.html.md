# css/css-mixins/local-var-substitution.html

```json
{
  "format_version": 3,
  "file": "css/css-mixins/local-var-substitution.html"
}
```

## style[0]

```css

    @function --f() {
      result: var(--unknown, PASS);
    }
    #target {
      --actual: --f();
      --expected: PASS;
    }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[1]

```css

    @function --f() {
      --x: var(--unknown);
      result: var(--x, PASS);
    }
    #target {
      --actual: --f();
      --expected: PASS;
    }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[2]

```css

    @function --f() {
      --x: PASS;
      result: var(--unknown, var(--x));
    }
    #target {
      --x: FAIL;
      --actual: --f();
      --expected: PASS;
    }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[3]

```css

    @function --f() {
      --y: PASS;
      --x: var(--unknown, var(--y));
      result: var(--x);
    }
    #target {
      --x: FAIL1;
      --Y: FAIL2;
      --actual: --f();
      --expected: PASS;
    }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
