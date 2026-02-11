# css/css-cascade/scope-style-sharing-001.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/scope-style-sharing-001.html"
}
```

## style[0]

```css

    @scope (.scope-start) to (.sibling + .sibling) {
      .foo {
        z-index: 1;
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

## style[1]

```css

    @scope (.scope-start) to (.sibling:not(.sibling + div)) {
      .foo {
        z-index: 1;
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

## style[2]

```css

    @scope (.sibling:not(.sibling + div)) {
      :scope {
        z-index: 1;
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

## style[3]

```css

    @scope (.sibling + .sibling) {
      :scope {
        z-index: 1;
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

## style[4]

```css

      @scope {
        :scope {
          z-index: 1;
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

## style[5]

```css

      @scope {
        :scope {
          z-index: 1;
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

## style[6]

```css

    @scope to (.sibling + .sibling) {
      .foo {
        z-index: 1;
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

## style[7]

```css

    @scope to (.sibling:not(.sibling + div)) {
      .foo {
        z-index: 1;
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

## style[8]

```css

    @scope (.sibling:has(> .foo)) {
      :scope {
        z-index: 1;
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

    @scope (.sibling:has(> .foo)) {
      :scope {
        z-index: 1;
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

    @scope (#first) {
      :scope {
        z-index: 1;
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

## style[11]

```css

    @scope (#second) {
      :scope {
        z-index: 1;
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

    @scope (.root) {
      > div:nth-of-type(2) {
        z-index: 1;
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
