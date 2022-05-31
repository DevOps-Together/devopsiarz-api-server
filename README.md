## Ważne

https://github.com/DevOps-Together/devopsiarz-api-server/issues - zapytania, problemy, plany, dzienniczek postępów itp

### Co robimy tutaj

Tworzymy prosty serwer API - serwer czeka na requesty typu POST i GET na jednym endpoincie: /wiadomosc

Czyli np. serwer na localhoście, ma url=localhost/wiadomosc

Do takiego serwera można uderzać narzędziami typu `curl` czy `httpie`

Dla przykładu, załóżmy, że serwer działa na hoście `localhost`

`http GET localhost/wiadomosc`

`http POST localhost/wiadomosc "tresc=ala_ma_kota"` - w tym przypadku treść wiadomości znajduje się w zmiennej `tresc`

### Minimalna funkcjonalność:

- POST /wiadomosc - dodaje tekst do 160 znaków (prześlij tekst dowolną zmienną)
- GET /wiadomosc - wyświetla listę zapisanych wiadomości (do tej pory)

### Sposób rozwiązywania
- tworzysz swój branch, na którym tworzysz rozwiązanie, kod na branchu nie musi działać dobrze, ani działać wcale - to jest nauka
- jak stworzysz branch - otwórz od razu Pull Request, aby na nim pracować i przyjmować uwagi
- jak nie wiesz, jak ruszyć - pytaj w Issues lub na kanale #projekt-devopsiarz na Discordzie
- język dowolny, jak chcesz się nauczyć, to zacznij w języku, w który chcesz wejść (kiedyś trzeba)

### Pomocne linki - tutaj rzucamy linki, które mogą pomóc nam lub innym
[Jak tworzyć Pull Request na GitHub - EN](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/creating-a-pull-request)
