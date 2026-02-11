# css/css-cascade/scope-hover.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/scope-hover.html"
}
```

## style[0]

```css

        @scope (.a:hover) {
          :scope { z-index: 1; }
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

        @scope (.a:hover) {
          :scope .b { z-index: 1; }
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

        @scope (.a) to (:scope:hover) {
          :scope { z-index: 1; }
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

        @scope (.a) to (.b:hover) {
          .b { z-index: 1; }
        }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
