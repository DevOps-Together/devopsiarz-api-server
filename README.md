## Ważne

https://github.com/DevOps-Together/devopsiarz-api-server/issues - zapytania, problemy, plany, dzienniczek postępów itp

Zacząłeś/aś pracę na swoim branchu? Otwórz od razu DRAFT Pull Request, by inni mogli pytać, doradzać, komentować: https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/creating-a-pull-request

### Co robimy tutaj

Tworzymy prosty serwer API - ten serwer czeka na requesty typu POST i GET na jednym endpoincie: /wiadomosc - przyjmuje wiadomości, zapisuje je jakoś oraz zwraca. Nic ponad taką funkcjonalność.

Czyli np. serwer postawiony na localhoście, będzie miał adres `localhost/wiadomosc`

Do takiego serwera można uderzać narzędziami typu `curl`, `httpie`, `postman` czy przeglądarką (zwłaszcza jak chodzi o metodę GET)

Dla przykładu, załóżmy, że serwer działa na hoście `localhost` i chcemy użyć narzędzia `httpie` do wysyłania requestów, wtedy moglibyśmy to zrobić tak:

`http GET localhost/wiadomosc` - dostaniemy listę zapisanych wiadomości

`http POST localhost/wiadomosc "tresc=ala_ma_kota"` - zapiszemy wiadomość `ala_ma_kota`, w tym przypadku treść wiadomości znajduje się w zmiennej `tresc`, a metodą użytą do zapisu jest metoda `POST`

Trochę o metodach HTTP: https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods

To w jaki sposób serwer ma zapisywać wiadomości, zostawiam Tobie. Jeśli przykładowo, chcesz nauczyć się trochę PostgreSQLa - musisz oprogramować używanie PostgreSQLa, czyli zapisywanie w nim danych i pobieranie. Jak nie wiesz w ogóle jak odpalić PostgreSQLa, to możesz użyć Dockera. Nie znasz Dockera? To zatem idealna okazja, aby poznać. :) W razie czego, pamiętaj: https://github.com/DevOps-Together/devopsiarz-api-server/issues

Bazą danych może być też pamięć RAM, czyli "trzymanie" stanu przez program (do jego wyłączenia lub padu). Jednak jeśli chcesz uczyć się trochę baz danych, to przy okazji tego projektu jest szansa w nie wejść.

### Minimalna funkcjonalność:

- POST /wiadomosc - dodaje tekst do 160 znaków (prześlij tekst dowolną zmienną, w przykładzie jest `tresc` ale to nie jest wymóg)
- GET /wiadomosc - wyświetla listę zapisanych wiadomości (do tej pory)

### Sposób rozwiązywania
- tworzysz swój branch, na którym tworzysz rozwiązanie, kod na branchu nie musi działać dobrze, ani działać wcale - to jest nauka
- jak stworzysz branch - **otwórz od razu Pull Request**, aby na nim pracować i przyjmować uwagi
- jak nie wiesz, jak ruszyć - pytaj w Issues lub na kanale #projekt-devopsiarz na Discordzie
- język dowolny, jak chcesz się nauczyć, to zacznij w języku, w który chcesz wejść (kiedyś trzeba)

### Pomocne linki - tutaj rzucamy linki, które mogą pomóc nam lub innym
[Jak tworzyć Pull Request na GitHub - EN](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/creating-a-pull-request)
