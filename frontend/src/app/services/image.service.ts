import { Injectable } from "@angular/core";
import { Observable } from "rxjs";
import { HttpClient } from "@angular/common/http";
import { environment } from "../../environments/environment";


@Injectable({
  providedIn: 'root'
})
export class ImageService {
  private baseUrl: string = `${environment.serverUrl}/api/v1/image`

  constructor(private http: HttpClient) {}

  public get(id: String): Observable<Blob> {
    return this.http.get<Blob>(this.baseUrl + `/${id}`, {responseType: 'blob' as 'json'});
  }
}
