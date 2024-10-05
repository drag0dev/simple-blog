import { Injectable } from "@angular/core";
import { CreateBlogPostDTO } from "../models/create-blogpost-dto.model";
import { Observable } from "rxjs";
import { HttpClient } from "@angular/common/http";


@Injectable({
  providedIn: 'root'
})
export class BlogpostService {
  private baseUrl: string = 'http://localhost:8080/blogpost'

  constructor(private http: HttpClient) {}

  public create(dto: CreateBlogPostDTO, postImage: Blob | null): Observable<null> {
    const formData = new FormData();

    formData.append('data', JSON.stringify(dto));
    if (postImage != null) { formData.append('image', postImage, 'image.png'); }

    return this.http.post<null>(this.baseUrl, formData);
  }
}
