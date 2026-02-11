# css/mediaqueries/mq-invalid-media-type-006.html

```json
{
  "format_version": 3,
  "file": "css/mediaqueries/mq-invalid-media-type-006.html"
}
```

## style[0]

```css

        div {
            width: 100px;
            height: 20px;
        }

        #valid1,
        #valid2,
        #valid3 {
            background-color: red;
        }

        #invalid1,
        #invalid2 {
            background-color: green;
        }

        @media screen and (min-width: 480px), screen and (device-width: 768px) {
            #valid1 {
                background-color: green;
            }
        }

        @media screen {
            #valid2 {
                background-color: green;
            }
        }

        @media {
            #valid3 {
                background-color: green;
            }
        }

        @media screen and (min-width: 480px) screen and (device-width: 768px) {
            #invalid1 {
                background-color: red;
            }
        }

        @media screen and (min-width: 480px) and (device-width: 768px) {
            #invalid2 {
                background-color: red;
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
