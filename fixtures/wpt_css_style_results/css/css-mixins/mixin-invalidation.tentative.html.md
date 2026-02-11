# css/css-mixins/mixin-invalidation.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-mixins/mixin-invalidation.tentative.html"
}
```

## style[0]

```css

       @mixin --m1() {
         @result {
           color: red;
           & {
             --foo: bar;
           }
         }
       }
       @mixin --m2() {
         @result {
           color: red;
         }
       }
       #target1 {
         @apply --m1;
       }
     
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[1]

```css

       #target2 {
         @apply --m2;
       }
     
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[2]

```css

       @mixin --m3() {
         @result {
           color: green;
         }
       }
       #target3 {
         color: red;
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
