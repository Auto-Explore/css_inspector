# css/css-mixins/mixin-parameters.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-mixins/mixin-parameters.tentative.html"
}
```

## style[0]

```css

    @mixin --m1() {
      @result {
        color: green;
      }
    }
    @mixin --m1(junk-in-parameter-list) {  /* Will be ignored. */
      @result {
        color: red;
      }
    }
    #e1 {
      @apply --m1;
    }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[1]

```css

    @mixin --m2(--my-color) {
      @result {
        color: var(--my-color);
      }
    }
    #e2 {
      @apply --m2(green);
    }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[2]

```css

    @mixin --m2b(--my-color) {
      @result {
        color: var(--my-color);
      }
    }
    #e2b {
      @apply --m2b(navy);
    }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[3]

```css

    @mixin --m2c(--my-color) {
      @result {
        color: var(--my-color);
      }
    }
    #e2c {
      color: fuchsia;
      @apply --m2c();
    }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[4]

```css

    @mixin --m3(--my-color) {
      @result {
        color: var(--my-color);
      }
    }
    #e3 {
      @apply --m3({green});
    }
  
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[5]

```css

    @mixin --m4(--my-color: green) {
      @result {
        color: var(--my-color);
      }
    }
    #e4 {
      @apply --m4;
    }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[6]

```css

    @mixin --m5() {
      @result {
        color: red;
      }
    }
    #e5 {
      color: green;
      @apply --m5(too-many-parameters);
    }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[7]

```css

    @mixin --m6(--my-color) {
      @result {
        color: var(--my-color, navy);
      }
    }
    #e6 {
      @apply --m6(green);
    }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[8]

```css

    @mixin --m6b(--my-color) {
      @result {
        color: var(--my-color, navy);
      }
    }
    #e6b {
      @apply --m6b;
    }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[9]

```css

    @mixin --m6c(--my-color) {
      @result {
        color: var(--my-color, navy);
      }
    }
    #e6c {
      @apply --m6c(invalid-color);
    }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[10]

```css

    #e6d {
      color: var(--not-within-mixin, green);
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

    #e6e {
      color: var(--not-within-mixin);
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

    @mixin --m7(--my-color) {
      @result {
        color: attr(color-attr type(<color>));
      }
    }
    #e7 {
      @apply --m7(green);
    }
  
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[13]

```css

    @mixin --m8(--my-length type(<length>)) {
      @result {
        font-size: 10px;
        --some-length: var(--my-length);
      }
    }
    #e8 {
      @apply --m8(1.0em);
    }
  
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[14]

```css

    @mixin --m9(--my-length type(length)) {  /* Note the syntax error. */
      @result {
        font-size: 10px;
        --some-length: var(--my-length);
      }
    }
    #e9 {
      @apply --m9(1.0em);
    }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[15]

```css

    @mixin --m10inner(--inner-color) {
      @result {
        color: var(--outer-color);
      }
    }
    @mixin --m10outer(--outer-color) {
      @result {
        @apply --m10inner(red);
      }
    }
    #e10 {
      @apply --m10outer(green);
    }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[16]

```css

    @mixin --m11(--val, --true, --false) {
      @result {
        color: if(style(--x: var(--val)): var(--true); else: var(--false))
      }
    }
    #e11 {
      --x: a;
      @apply --m11(a, green, red);
    }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[17]

```css

    @mixin --m11b(--val, --true, --false) {
      @result {
        color: if(style(--x: var(--val)): var(--true); else: var(--false))
      }
    }
    #e11b {
      --x: a;
      @apply --m11b(b, red, green);
    }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[18]

```css

    @function --f() {
      color: var(--my-color);
    }
    @mixin --m12(--my-color) {
      @result {
        color: --f();
      }
    }
    #e12 {
      @apply --m12(red);
    }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[19]

```css

  @mixin --m13-override-outer(--inner, --outer) {
    @result {
      color: var(--inner);
    }
  }
  @mixin --m13-set-inner(--inner) {
    @result {
      @apply --m13-override-outer(var(--inner), red);
    }
  }
  @mixin --m13(--outer) {
    @result {
      @apply --m13-set-inner(var(--outer));
    }
  }
  #e13 {
    @apply --m13(green);
  }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[20]

```css

    @mixin --m14(--param: green) {
      @result {
        color: var(--param);
      }
    }
    #e14 {
      @apply --m14();
    }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[21]

```css

    @mixin --m15(--my-length type(<length>): 1em) {
      @result {
        --some-length: var(--my-length);
      }
    }
    #e15 {
      font-size: 12px;
      @apply --m15();
    }
  
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[22]

```css

    @mixin --m16(--arg: !!) {  /* Invalid declaration. */
      @result {
        color: red;
      }
    }
    #e16 {
      color: green;
      @apply --m16();
    }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[23]

```css

    @mixin --m17(--arg) {
      @result {
        color: red;
      }
    }
    #e17 {
      color: green;
      @apply --m17(!!!);  /* Invalid invocation. */
    }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
