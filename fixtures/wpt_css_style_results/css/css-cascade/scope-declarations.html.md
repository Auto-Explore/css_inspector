# css/css-cascade/scope-declarations.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/scope-declarations.html"
}
```

## style[0]

```css

    @scope (.a) {
      z-index: 1;
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

      @scope {
        z-index: 1;
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
      :where(:scope) {
        z-index: 1;
      }
      z-index: 2; /* Wins due to order */
    }
    @scope (.b) {
      z-index: 1;
      :where(:scope) {
        z-index: 2; /* Wins due to order */
      }
    }
    @scope (.c) {
      :scope {
        z-index: 1; /* Wins due to specificity */
      }
      z-index: 2;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
