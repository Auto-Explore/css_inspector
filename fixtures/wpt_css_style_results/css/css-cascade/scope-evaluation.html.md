# css/css-cascade/scope-evaluation.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/scope-evaluation.html"
}
```

## style[0]

```css

  :where(main *) {
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

    @scope (.a) {
      .a { background-color: green; }
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

    @scope (.a) to (.c) {
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

    @scope (.a) {
      :scope > span { background-color: green; }
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

    @scope (.a) to (:scope > .b) {
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

    @scope (.a) to (:scope > .b) {
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

    @scope (.a) {
      @scope (:scope > .b) {
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

## style[9]

```css

    @scope (.a) to (:scope > .b) {
      .c { background-color: green; }
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

    @scope (.a) to (.b) {
      .c { background-color: green; }
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

## style[12]

```css

    @scope (.b) {
      @scope (.a) {
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

## style[13]

```css

    @scope (.a) {
      @scope (.b) to (.c) {
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

## style[14]

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

## style[15]

```css

    @scope (.a) to (.b) {
      * { background-color: green; }
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

    @scope (.a) to (.b > *) {
      * { background-color: green; }
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

    @scope (.a) to (:scope) {
      * { background-color: green; }
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

    @scope (.a) {
      :scope + .c { background-color: green; }
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

    @scope (.a) {
      :scope + .c { background-color: green; }
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

    @scope (.a) {
      > span { background-color: green; }
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

    @scope (.a) {
      /* Can never match anything. */
      :scope > :scope { background-color: green; }
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

## style[23]

```css

    @scope (.a) to (.b, .c) {
      * { background-color:green; }
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
