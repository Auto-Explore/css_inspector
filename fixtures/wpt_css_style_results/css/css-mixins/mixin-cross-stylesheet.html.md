# css/css-mixins/mixin-cross-stylesheet.html

```json
{
  "format_version": 3,
  "file": "css/css-mixins/mixin-cross-stylesheet.html"
}
```

## style[0]

```css

      div {
        color: red;
        @apply --m1;
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

      @mixin --m1() {
        @result {
          color: green;
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
