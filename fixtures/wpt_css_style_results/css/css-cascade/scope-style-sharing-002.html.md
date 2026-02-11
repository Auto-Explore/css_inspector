# css/css-cascade/scope-style-sharing-002.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/scope-style-sharing-002.html"
}
```

## style[0]

```css

    @scope (.scope-start) to (:scope:last-of-type > .foo) {
      .foo {
        z-index: 1;
      }
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

    @scope (.scope-start) to (:scope:first-of-type > .foo) {
      .foo {
        z-index: 1;
      }
    }
    .scope-start:first-child {
      color: green;
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

    @scope (.scope-start:first-of-type > .foo) {
      :scope {
        z-index: 1;
      }
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

    @scope (.scope-start:last-of-type > .foo) {
      :scope {
        z-index: 1;
      }
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

## style[4]

```css

      @scope {
        .foo {
          z-index: 1;
        }
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

## style[5]

```css

      @scope {
        .foo {
          z-index: 1;
        }
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

## style[6]

```css

        @scope {
          :scope {
            z-index: 1;
          }
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

## style[7]

```css

        @scope {
          :scope {
            z-index: 1;
          }
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

## style[8]

```css

    @scope (.scope-start:has(> .bar)) {
      .foo {
        z-index: 1;
      }
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

## style[9]

```css

    @scope (.scope-start:has(> .bar)) {
      .foo {
        z-index: 1;
      }
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

## style[10]

```css

    @scope (#first-parent) {
      .foo {
        z-index: 1;
      }
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

## style[11]

```css

    @scope (#second-parent) {
      .foo {
        z-index: 1;
      }
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

## style[12]

```css

    @scope (.root) {
      > div:nth-of-type(2) > div {
        z-index: 1;
      }
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
