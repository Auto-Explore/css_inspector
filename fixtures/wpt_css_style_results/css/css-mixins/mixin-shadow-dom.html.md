# css/css-mixins/mixin-shadow-dom.html

```json
{
  "format_version": 3,
  "file": "css/css-mixins/mixin-shadow-dom.html"
}
```

## style[0]

```css

      @mixin --exists-only-outside-shadow() {
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

## style[1]

```css

          #e1 {
            color: red;
            @apply --exists-only-outside-shadow;
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

          #e2 {
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

## style[3]

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

## style[4]

```css

    
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

          #e3 {
            color: red;
            @apply --exists-only-in-adopted;
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

          @mixin --in-shadow() {
            @result {
              color: red;
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

      #e4 {
        color: green;
        @apply --in-shadow;
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
