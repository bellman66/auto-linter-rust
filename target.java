package io.re100.consulting.domain.content.web.repository.extension;

import io.re100.consulting.domain.content.data.dto.AdvancedConsultingExpressionDto;
import io.re100.consulting.domain.content.data.dto.AdvancedConsultingFilterDto;
import io.re100.consulting.domain.content.data.dto.VisitConsultingPagingDto;
import io.re100.consulting.domain.content.data.enums.ConsultingSortType;
import java.util.List;
import org.springframework.data.domain.PageRequest;

public interface VisitConsultingRepositoryExtension {

    boolean existsConsultingByExpression(AdvancedConsultingExpressionDto build);

    long getTotalCount(Long accountId);

    long getTotalCount(AdvancedConsultingFilterDto consultingFilterDto);

    List<Long> findIdGroupByPageRequest(PageRequest pageRequest, AdvancedConsultingFilterDto consultingFilterDto, ConsultingSortType sortType);

    List<Long> findIdGroupByPageRequest(PageRequest pageRequest, Long accountId);

    List<VisitConsultingPagingDto> getConsultingPage(List<Long> idGroup);

    List<VisitConsultingPagingDto> getConsultingPageAtAdminAccount(List<Long> idGroup);

    List<VisitConsultingPagingDto> getAdminConsultingPage(List<Long> idGroup, ConsultingSortType sortType);

}
