# css/css-mixins/mixin-cycle.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-mixins/mixin-cycle.tentative.html"
}
```

## style[0]

```css

       @mixin --a() {
         @result {
           --last-processed: a;
           @apply --b;
         }
       }
       @mixin --b() {
         @result {
           --last-processed: b;
           @apply --c;
         }
       }
       @mixin --c() {
         @result {
           --last-processed: c;
           @apply --b;
         }
       }
       #target {
         @apply --a;
       }
     
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
