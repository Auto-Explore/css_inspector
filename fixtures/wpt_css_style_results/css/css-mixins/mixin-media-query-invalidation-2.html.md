# css/css-mixins/mixin-media-query-invalidation-2.html

```json
{
  "format_version": 3,
  "file": "css/css-mixins/mixin-media-query-invalidation-2.html"
}
```

## style[0]

```css

        @mixin --m1() {
          @result {
            color: red;
          }
        }
        @media (width < 50px) {
          @mixin --m1() {
            @result {
              color: green;
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

## style[1]

```css

        /* Invalidated by a mixin in a different style sheet. */
        #target {
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
