# css/css-cascade/scope-focus.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/scope-focus.html"
}
```

## style[0]

```css

        @scope (.a:focus) {
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

        @scope (.a:focus) {
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

        @scope (.a) to (:scope:focus) {
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

        @scope (.a) to (.b:focus) {
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
