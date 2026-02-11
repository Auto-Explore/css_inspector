# css/css-cascade/scope-invalidation.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/scope-invalidation.html"
}
```

## style[0]

```css

  main * {
    background-color: black;
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

    @scope (.a) {
      span { background-color: green; }
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

    @scope (.a, .b) {
      span { background-color: green; }
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

    @scope (.a) {
      :scope { background-color: green; }
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

    @scope (.a) to (.b) {
      span { background-color: green; }
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

    @scope (.a) to (.b, .c) {
      span { background-color: green; }
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

    @scope (.a) to (.b) {
      span { background-color: green; }
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

    @scope (.a) to (.b .c) {
      span { background-color: green; }
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

    @scope (.a) to (.b ~ .c) {
      span { background-color: green; }
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

    @scope (.a) {
      @scope (.b) {
        span { background-color: green; }
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

## style[10]

```css

    @scope (.a) {
      :scope { background-color:green; }
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

    @scope (.a:has(.c)) {
      .b { background-color:green; }
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

    @scope (.a:has(.c)) {
      :scope { background-color:green; }
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

    @scope (.a:has(.c)) {
      :scope { background-color:green; }
      :scope .b { background-color:green; }
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

    @scope (.a) to (.b:has(.c)) {
      .b { background-color:green; }
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

    @scope (.a) {
      .b ~ :scope { background-color:green; }
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

    @scope (.a ~ .b) {
      .c { background-color:green; }
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

    @scope (.a + .b) {
      .c { background-color:green; }
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

    @scope (.root) {
      :not(:scope) { background-color:green; }
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

    @scope (.root) {
      :not(:scope) > .a { background-color:green; }
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

    @scope (.root) to (:not(:scope)) {
      :is(div, :scope) { background-color: green; }
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

    @scope (.root) to (:not(:scope) > .a) {
      :is(div, :scope) { background-color: green; }
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

    @scope (:nth-child(2n of .a)) {
      :scope { background-color: green; }
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

    @scope (#wrapper) to (:nth-child(4n of .a)) {
      div { background-color: green; }
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

    @scope (.a) {
      .nomatch { background-color: green; }
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

    @scope (.a) {
      .nomatch { background-color: green; }
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

    .a {
      > .b, > .c {
        background-color: green; /* Specificity: (0, 2, 0) */
      }
    }
    @scope (.a.a) {
      .nomatch1 {
        background-color: red; /* Specificity: (0, 1, 0) */
      }
      .nomatch2 {
        background-color: red; /* Specificity: (0, 1, 0) */
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

## style[27]

```css

    @scope (.a .b) {
      @scope(.c .d, .e .f) {
        .g {
          background-color: green;
        }
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

## style[28]

```css

    @scope (.a) to (.b .c) {
      :scope .d .f{
        background-color: green;
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

## style[29]

```css

        @scope(.a) {
          @scope {
            @scope(> .b) {
              :scope { background: green; }
            }
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
